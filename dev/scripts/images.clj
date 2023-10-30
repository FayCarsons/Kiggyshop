#!/usr/bin/env bb

(require '[babashka.fs :as fs]
         '[babashka.cli :refer [parse-opts]]
         '[clojure.string :as s]
         '[clojure.java.shell :refer [sh]])

(def cli-opts {:target :string})
(def args (parse-opts *command-line-args* {:spec cli-opts}))
(def src-dir (:src args))
(def target-dir (:target args))

; Delete all files within the target directory if it exists, otherwise create it
(if (fs/exists? target-dir)
  (doseq [file (fs/list-dir target-dir)]
    (fs/delete file))
  (fs/create-dir target-dir))


(def input-files (s/split (apply str (rest (second (sh "ls" src-dir)))) #"\s"))
(doseq [input-file input-files]
  (println "current file: " input-file)
  (let [filename (s/replace input-file #"\s" "")
        filename-without-extension (first (s/split filename #"\."))]
    ; Convert to Thumbnail (JPEG)
    (sh "ffmpeg" "-i"
        (str src-dir "/" input-file)
        "-vf" "scale=100:-1"
        (str target-dir "/" filename-without-extension "_thumbnail.jpg"))

    ; Convert to Medium (AVIF)
    (sh "ffmpeg" "-i"
        (str src-dir "/" input-file)
        "-vf" "scale=500:-1"
        (str target-dir "/" filename-without-extension "_medium.avif"))

    ; Convert to Large (WebP)
    (sh "ffmpeg" "-i"
        (str src-dir "/" input-file)
        (str target-dir "/" filename-without-extension "_large.webp"))))

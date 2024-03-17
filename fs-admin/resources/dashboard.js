"use strict";

// GLOBALS AND ENUMS
let currentTab = null;
let stock = {};

const tab_to_str = (tab) => {
  return (
    {
      [tabState.stock]: "stock",
      [tabState.orders]: "orders",
    }[tab] ?? fail()
  );
};

const actionToMethod = (action) => {
  return {
    [itemAction.add]: "PUT",
    [itemAction.edit]: "PUT",
    [itemAction.delete]: "DELETE",
  }[action];
};

const actionToUrl = (action, id = null) => {
  return (
    {
      [[itemAction.add, false]]: "/api/stock",
      [[itemAction.edit, true]]: `/api/stock/${id}`,
      [[itemAction.delete, false]]: "/api/stock",
    }[[action, !!id]] ?? fail()
  );
};

const actionToStr = (action) => {
  return (
    {
      [itemAction.edit]: "edit",
      [itemAction.add]: "add",
      [itemAction.delete]: "delete",
    }[action] ?? fail()
  );
};

const fail = () => {
  alert("Something has gone horribly wrong, call your girlfriend");
};

const orderFilterDropDown = document.getElementById("orderFilter");
orderFilterDropDown.addEventListener("change", () => {
  showOrders(orderFilterDropDown.value);
});

const showTab = async (tab) => {
  if (
    (tab == tabState.stock && currentTab == tabState.stock) ||
    (tab == tabState.orders && currentTab == tabState.orders)
  ) {
    return;
  }

  var tabs = document.querySelectorAll('[id$="Tab"]');
  for (var i = 0; i < tabs.length; i++) {
    tabs[i].style.display = "none";
  }

  document.getElementById(`${tab_to_str(tab)}Tab`).style.display = "block";

  if (tab == tabState.stock) {
    currentTab = tabState.stock;
    showStock();
  } else if (tab == tabState.orders) {
    currentTab = tabState.orders;
    showOrders("All");
  }
};

const initStock = async () => {
  if (Object.entries(stock).length > 0) stock = {};
  try {
    const res = await fetch("/api/stock", {
      method: "GET",
      headers: {
        "Content-type": "application/json",
      },
    });

    stock = (await res.json()) ?? {};
  } catch (err) {
    alert(err);
  }
};

const showStock = async () => {
  if (Object.keys(stock).length === 0) {
    await initStock();
  }

  const stockList = document.getElementById("stockList");
  stockList.innerHTML = "";

  for (let [id, item] of Object.entries(stock)) {
    id = parseInt(id);
    const { title, kind, description, quantity } = item;
    const row = document.createElement("tr");
    // Minifier doesn't catch the linefeeds in this expression so have to do it manually
    row.innerHTML = `<td><input type="checkbox" id="${id}"></td><td>${title.toUpperCase()}</td><td>${kind}</td><td>${description}</td><td>${quantity}</td><td><button onclick="showItemModal(itemAction.edit, ${id})">Edit</button></td>`;
    stockList.appendChild(row);
  }
};

const showOrders = async (filter) => {
  filter = filter.trim();

  try {
    const res = await fetch(`/api/orders/${filter}`, {
      method: "GET",
      headers: {
        "Content-type": "application/json",
      },
    });

    if (!res.ok) throw new Error(`Cannot fetch order: ${res.statusText}`);

    const orders = await res.json();

    updateOrderUI(orders);
  } catch (err) {
    alert(err);
  }
};

const updateOrderUI = (orders) => {
  const orderList = document.getElementById("orderList");
  orderList.innerHTML = "";

  orders.forEach((order) => {
    const row = document.createElement("tr");
    row.innerHTML = `<td>${order.id}</td><td>${order.name}</td><td>${order.street}</td><td>${order.zipcode}</td><td>${order.fulfilled}</td>`;
    orderList.appendChild(row);
  });
};

const deleteSelectedStock = async () => {
  const checkboxes = document.querySelectorAll(
    'input[type="checkbox"]:checked'
  );
  const stockIds = Array.from(checkboxes).map((item) => parseInt(item.id));
  if (stockIds.length === 0) return;
  doItemAction(itemAction.delete, stockIds);
};

const showItemModal = (mode, id) => {
  const modal = document.getElementById("itemModal");
  modal.style.display = "block";

  const button = document.getElementById("submitItem");
  button.onclick =
    mode === itemAction.edit ? () => submitEdited(id) : addItem;

  if (mode == itemAction.edit) {
    displayItem(id);
  }
};

const addItem = () => {
  const item = itemFromForm();
  const imageInput = document.getElementById("imageInput");

  if (imageInput.files.length > 0) {
    uploadImage(imageInput.files[0], item.title);
    doItemAction(itemAction.add, item);
    closeModal();
  } else {
    alert("U forgor a image :P");
  }
};

const displayItem = async (itemId) => {
  if (stock.size === 0) {
    fail();
  }
  const { title, kind, description, quantity } = stock[itemId];

  document.getElementById("title").value = title;
  document.getElementById("kind").value = kind;
  document.getElementById("description").value = description;
  document.getElementById("quantity").value = quantity;

  const image = document.getElementById("image");
  image.src = `/api/resources/images/${title.trim().replace(" ", "")}.png`;
  image.style.display = "block";
};

const itemFromForm = () => {
  const title = document.getElementById("title").value;
  const kind = document.getElementById("kind").value;
  const description = document.getElementById("description").value;
  const quantity = parseInt(document.getElementById("quantity").value);
  let item = {
    title: title,
    kind: kind,
    description: description,
    quantity: quantity,
  };

  if (validateItem(item)) {
    return item;
  } else {
    alert("Pwease dubbo check ur input, somefing was wrong :P");
  }
};

const validateItem = ({ title, kind, description, quantity }) => {
  return (
    title &&
    kind &&
    description &&
    quantity > -1 &&
    typeof quantity === "number"
  );
};

const submitEdited = (itemId) => {
  const item = itemFromForm();
  const imageInput = document.getElementById("imageInput");

  doItemAction(itemAction.edit, item, itemId);
  if (imageInput.files.length === 1) uploadImage(imageInput.files[0], title);
};

const doItemAction = async (action, body = null, id = null) => {
  try {
    let res = await fetch(actionToUrl(action, id), {
      method: actionToMethod(action),
      headers: {
        "Content-type": "application/json",
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      throw new Error(
        `Failed to ${actionToStr(action)} item(s): ${res.statusText}`
      );
    }
    closeModal();
    initStock();
    showStock();
  } catch (err) {
    alert(err);
  }
};

const closeModal = () => {
  const modal = document.getElementById("itemModal");
  modal.style.display = "none";
  const img = document.getElementById("image");
  img.style.display = "none";
  img.src = "";
  clear_modal();
  clearForm();
};

const uploadImage = async (imageFile, title) => {
  const imgReader = new FileReader();
  imgReader.onload = async (event) => {
    try {
      const data = new Uint8Array(event.target.result);
      const res = await fetch(`/admin/upload_image/${title}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/octet-stream",
        },
        body: data,
      });

      if (!res.ok) {
        throw new Error(`Unable to upload image: ${res.statusText}`);
      }
    } catch (err) {
      console.error(err);
      alert("Somefin went wong w the image :P");
    }
  };

  imgReader.readAsArrayBuffer(imageFile);
};

const clear_modal = () => {
  document.getElementById("title").value = "";
  document.getElementById("kind").value = "";
  document.getElementById("description").value = "";
  document.getElementById("quantity").value = "";
};

const clearForm = () => {
  const imageForm = document.getElementById("imageInput");
  const clone = imageForm.cloneNode(true);

  imageForm.parentNode.replaceChild(clone, imageForm);

  clone.value = "";
};

showTab(tabState.stock);

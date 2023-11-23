"use strict";

// GLOBALS AND ENUMS
let currentTab = null;
let stock = new Map();

const tab_to_str = (tab) => {
  if (tab == tabState.stock) {
    return "stock";
  } else if (tab == tabState.orders) {
    return "orders";
  } else {
    app_error();
  }
};

const action_to_method = (action) => {
    switch(action) {
        case itemAction.add:
            return 'PUT'
        case itemAction.edit: 
            return 'PUT'
        case itemAction.delete:
            return 'DELETE'
        default:
            app_error()
    }
};

const action_to_url = (action, id = null) => {
  if (action == itemAction.add && id === null) {
    return "/api/stock/add";
  } else if (action == itemAction.edit && id) {
    return `/api/stock/update/${id}`;
  } else if (action == itemAction.delete && id === null) {
    return `/api/stock/delete`;
  } else {
    app_error();
  }
};

const action_to_str = (action) => {
  switch (action) {
    case itemAction.edit:
      return "edit";
    case itemAction.add:
      return "add";
    case itemAction.delete:
      return "delete";
  }
};

const app_error = () => {
  alert("Something has gone horribly wrong, call your girlfriend");
};

const orderFilterDropDown = document.getElementById("orderFilter");
orderFilterDropDown.addEventListener("change", () => {
  show_orders(orderFilterDropDown.value);
});

const show_tab = async (tab) => {
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
    show_stock();
  } else if (tab == tabState.orders) {
    currentTab = tabState.orders;
    show_orders("All");
  }
};

const init_stock = async () => {
  try {
    const res = await fetch("/api/stock/get", {
      method: "GET",
      headers: {
        "Content-type": "application/json",
      },
    });

    const stock_res = await res.json();
    console.log(stock_res)
    for (let item of stock_res) {
      stock.set(item.id, item);
    }
  } catch (err) {
    alert(err);
  }
};

const show_stock = async () => {
  if (stock.size === 0) {
    await init_stock();
  }

  const stockList = document.getElementById("stockList");
  stockList.innerHTML = "";

  for (let [id, { title, kind, description, quantity }] of stock) {
    id = parseInt(id);
    const row = document.createElement("tr");
    row.innerHTML = `
                    <td><input type="checkbox" id="${id}"></td>
                    <td>${title.toUpperCase()}</td>
                    <td>${kind}</td>
                    <td>${description}</td>
                    <td>${quantity}</td>
                    <td><button onclick="show_item_modal(itemAction.edit, ${id})">Edit</button></td>
                    `;
    stockList.appendChild(row);
  }
};

const show_orders = async (filter) => {
  filter = filter.trim();

  try {
    const res = await fetch(`/api/orders/get/${filter}`, {
      method: "POST",
      headers: {
        "Content-type": "application/json",
      },
    });

    if (!res.ok) throw new Error(`Cannot fetch order: ${res.statusText}`);

    const orders = await res.json();

    update_order_UI(orders);
  } catch (err) {
    alert(err);
  }
};

const update_order_UI = (orders) => {
  const orderList = document.getElementById("orderList");
  orderList.innerHTML = "";

  orders.forEach((order) => {
    const row = document.createElement("tr");
    row.innerHTML = `
                    <td>${order.id}</td>
                    <td>${order.name}</td>
                    <td>${order.street}</td>
                    <td>${order.zipcode}</td>
                    <td>${order.fulfilled}</td>
                `;
    orderList.appendChild(row);
  });
};

const delete_selected_stock = async () => {
  const checkboxes = document.querySelectorAll(
    'input[type="checkbox"]:checked'
  );
  const stock_ids = Array.from(checkboxes).map((item) => parseInt(item.id));
  if (stock_ids.length === 0) return;
  do_item_action(itemAction.delete, stock_ids);
};

const show_item_modal = (mode, id) => {
  const modal = document.getElementById("itemModal");
  modal.style.display = "block";

  const button = document.getElementById("submitItem");
  button.onclick =
    mode === itemAction.edit ? () => submit_edited(id) : add_item;

  if (mode == itemAction.edit) {
    display_item(id);
  }
};

const add_item = () => {
  const item = item_from_form();
  const image_input = document.getElementById("imageInput");

  if (image_input.files.length > 0) {
    uploadImage(image_input.files[0], item.title);
    do_item_action(itemAction.add, item);
    close_modal();
  } else {
    alert("U forgor a image :P");
  }
};

const display_item = async (item_id) => {
  if (stock.size === 0) {
    app_error();
  }
  const { title, kind, description, quantity } = stock.get(item_id);

  document.getElementById("title").value = title;
  document.getElementById("kind").value = kind;
  document.getElementById("description").value = description;
  document.getElementById("quantity").value = quantity;

  const image = document.getElementById("image");
  image.src = `/api/resources/images/${title.trim().replace(" ", "")}.png`;
  image.style.display = "block";
};

const item_from_form = () => {
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

  if (validate_item(item)) {
    return item;
  } else {
    alert("Pwease dubbo check ur input, somefing was wrong :P");
  }
};

const validate_item = ({ title, kind, description, quantity }) => {
    return title && kind && description && quantity > -1 && typeof quantity === 'number';
  };

const submit_edited = (item_id) => {
  const item = item_from_form();
  const image_input = document.getElementById("imageInput");

  do_item_action(itemAction.edit, item, item_id);
  if (image_input.files.length === 1) uploadImage(image_input.files[0], title);
};

const do_item_action = async (action, body = null, id = null) => {
  try {
    let res = await fetch(action_to_url(action, id), {
      method: action_to_method(action),
      headers: {
        "Content-type": "application/json",
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      throw new Error(
        `Failed to ${action_to_str(action)} item(s): ${res.statusText}`
      );
    }
    close_modal();
    show_stock();
  } catch (err) {
    alert(err);
  }
};

const close_modal = () => {
  const modal = document.getElementById("itemModal");
  modal.style.display = "none";
  const img = document.getElementById("image");
  img.style.display = "none";
  img.src = "";
  clear_modal();
  clear_form();
};

const uploadImage = async (imageFile, title) => {
  const imgReader = new FileReader();
  imgReader.onload = async (event) => {
    try {
        const data = new Uint8Array(event.target.result);
        const res = await fetch(`/admin/upload_image/${title}`, {
            method: "POST",
            headers: {
                'Content-Type': 'application/octet-stream',
            },
            body: data,
          });
        
          if (!res.ok) {
            throw new Error(`Unable to upload image: ${res.statusText}`)
          }
    } catch (err) {
        console.error(err)
        alert("Somefin went wong w the image :P")
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

const clear_form = () => {
  const imageForm = document.getElementById("imageInput");
  const clone = imageForm.cloneNode(true);

  imageForm.parentNode.replaceChild(clone, imageForm);

  clone.value = "";
};

show_tab(tabState.stock);

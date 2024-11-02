let api;
let product_id;

import * as client from './client.js';

window.addEventListener("load", () => {
    api = new client.PedalizeApi('http://127.0.0.1:9999');

    add_event_listeners();

    product_id = get_product_id();

    api.get_product(product_id).then(
        (p) => {
            if (p)
                write_new_product_informations(p)
        },
        console.error
    )

})


/**
 * Writes the new product information to the DOM.
 *
 * @param {Product} product - The new product object containing the following properties:
 * @param {string}   product.id          - The unique identifier of the product.
 * @param {string}   product.name        - The name of the product.
 * @param {Option<string>}   product.description - Optional. The description of the product.
 * @param {number}   product.price       - The price of the product.
 * @param {boolean}   product.size       - The size of the product.
 * @param {boolean}   product.wheel      - The wheel of the product.
 * @param {Option<string>}   product.main_image       - Optional. The URL of the product image.
 * @param {Option<string>}   product.second_image       - Optional. The URL of the product image.
 * @param {Option<string>}   product.third_image       - Optional. The URL of the product image.
 * @param {Option<string>}   product.fourth_image       - Optional. The URL of the product image.
 */
function write_new_product_informations(product){
    console.log(product)
    // update name
    document.getElementById("product_name").textContent = product.name;

    if (!product.size) {
        document.querySelector(".taille").style["display"] = "none";
    } else {
        delete document.querySelector(".taille").style["display"];

    }
    if (!product.wheel) {
        document.querySelector(".wheel_size").style["display"] = "none";
    } else {
        delete document.querySelector(".wheel_size").style["display"];
    }

    document.querySelector(".main_image img")
        .src = product.main_image ? `static/img/${product.main_image}` : "static/img/no-photo.png";

    if (product.second_image) {
        document.querySelector(".second_image").src = `static/img/${product.second_image}`
    } else {
        let child = document.querySelector(".second_image");
        child.parentNode.removeChild(child)
    }

    if (product.third_image) {
        document.querySelector(".third_image").src = `static/img/${product.third_image}`
    } else {
        let child = document.querySelector(".third_image");
        child.parentNode.removeChild(child)
    }

    if (product.fourth_image) {
        document.querySelector(".fourth_image").src = `static/img/${product.fourth_image}`
    } else {
        let child = document.querySelector(".fourth_image");
        child.parentNode.removeChild(child)
    }

    document.querySelector(".description")
        .textContent = product.description || "Aucune description n'a été fournie."



    // get product characteristics
    api.get_product_characteristics(product.id)
        .then(
            update_product_characteristics,
            console.error
        )
}

/**
 * Updates the product characteristics display on the page.
 *
 * @param {Array<Object>} characteristics - An array of objects representing the product characteristics.
 */
function update_product_characteristics(characteristics){
    document.querySelector(".spec_tab").innerHTML = '';
    if (characteristics.length > 0)
        characteristics.forEach((c, i) => set_product_characteristic(c, i < characteristics.length - 1))
    else
        document.querySelector(".spec_tab").innerHTML = '<h2>Aucune caractéristiques n\' a été précisée</h2>';
}

/**
 * Sets the product characteristic by appending the specified HTML content to the ".spec_tab" element.
 *
 * @param {Object} c - The product characteristic object.
 * @param {boolean} add_line - Determines whether to add a horizontal line after appending the content.
 * @return {void}
 */
function set_product_characteristic(c, add_line){
    document.querySelector(".spec_tab")
        .innerHTML += `<section class="content">
                <div class="question">
                    <p>${c.name}</p>
                </div>
                <div class="awnser">
                    <p>${c.detail}</p>
                </div>
            </section>`

    if (add_line) {
        document.querySelector(".spec_tab")
            .innerHTML += "<hr class='line'>"
    }
}

/**
 * Retrieves the product ID from the current URL.
 *
 * @return {string} The product ID extracted from the URL parameter named 'product'.
 */
function get_product_id(){
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get('product');
}


/**
 * Adds event listeners to specific elements on the page.
 * @return {void}
 */
function add_event_listeners(){
    document.querySelector(".add_cart")
        .addEventListener("click", add_to_cart)

    document.querySelector(".buy_now")
        .addEventListener("click", add_to_cart)
}

/**
 * Adds a product to the cart.
 *
 * @function add_to_cart
 * @void
 */
function add_to_cart(){
    api.add_article_to_cart(product_id).then(
        () => {
            window.location.href = "./cart.html";
        },
        console.error
    )
}
import * as client from './client.js';

let api;

addEventListener("load", () => {
    // TODO N'oubliez pas de changer cette variable!
    api = new client.PedalizeApi('http://127.0.0.1:9999');

    api.get_products().then(
        (products) => {
            if (products instanceof Array) {
                add_articles(products)
            } else {
                no_articles()
            }
        },
        console.error
    )
});

/**
 * Sets the innerHTML of the element with class name 'list' to display a message indicating that no articles match the search criteria.
 *
 * @returns {void}
 */
function no_articles(){
    let list = document.querySelector(".list");
    list.innerHTML = "<h2>Aucun article ne correspond à vos critères de recherche</h2>"
}

/**
 * Add articles to the list container.
 *
 * @param {Product[]} products - The articles to add to the list.
 */
function add_articles(products){
    let list = document.querySelector(".list");
    list.innerHTML = "";

    if (products.length < 1) {
        return no_articles()
    }

    products.forEach(add_article)
}

/**
 *
 * @param product
 */
function add_article(product) {
    console.log(product)
    let list = document.getElementById("product_list");

    let style = "";
    console.log(product, product.main_image)
    if (product.main_image) {
        style = `background: center no-repeat url('static/img/${product.main_image}')`
    }
    console.log(style)

    list.innerHTML += `<a href="product_page.html?product=${product.id}">
        <div class="product">
            <div class="img" style="${style}"></div>
            <h2>${product.name}</h2>
            <p>${product.price}€</p>
        </div>
    </a>`
}
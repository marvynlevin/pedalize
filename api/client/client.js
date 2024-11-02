var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
/**
 * Represent the Api interface between the client and the server
 */
class PedalizeApi {
    constructor(host) {
        this.config = { host };
    }
    /**
     * Retrieves a product with the given ID.
     *
     * @param {string} id - The ID of the product.
     * @returns {Promise<Option<Product>>} - A promise that resolves to an option containing the product, or an error.
     */
    get_product(id) {
        return __awaiter(this, void 0, void 0, function* () {
            const req = new RequestBuilder(`${this.config.host}/product/${id}`);
            req.set_method("GET");
            try {
                const res = yield req.send();
                return yield res.json();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
    /**
     * Retrieves a specific page of products from the server.
     *
     * @param {number} page - The page number to retrieve. Must be a positive or null integer.
     *
     * @returns {Promise<Option<Product[]>>} - A promise that resolves to an Option containing an array of Product objects on success, or an error on failure.
     *
     * @throws {Error} - If the page number is not a positive or null integer.
     */
    get_page(page) {
        return __awaiter(this, void 0, void 0, function* () {
            if (page < 0 && Math.floor(page) == page)
                throw new Error("The page must be a positive or null integer");
            const req = new RequestBuilder(`${this.config.host}/product/page?page=${page}`);
            req.set_method("GET");
            try {
                const res = yield req.send();
                return yield res.json();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
    /**
     * Retrieves all products from the server.
     *
     * @returns {Promise<Option<Product[]>>} - A promise that resolves to an array of products, or an error if the request fails.
     */
    get_products() {
        return __awaiter(this, void 0, void 0, function* () {
            const req = new RequestBuilder(`${this.config.host}/product/all`);
            req.set_method("GET");
            try {
                const res = yield req.send();
                return yield res.json();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
    /**
     * Retrieves the characteristics of a product specified by its ID.
     *
     * @param {string} id - The ID of the product.
     * @returns {Promise<Option<ProductCharacteristic[]>>} - A promise that resolves to an option of an array of product characteristics. If the retrieval is successful, the promise resolves to the characteristics. If there is an error, the promise resolves to the error object.
     */
    get_product_characteristics(id) {
        return __awaiter(this, void 0, void 0, function* () {
            const req = new RequestBuilder(`${this.config.host}/product/${id}/characteristics`);
            req.set_method("GET");
            try {
                const res = yield req.send();
                return yield res.json();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
    /**
     * Retrieves the reviews for a given product ID.
     *
     * @param {string} id - The ID of the product to retrieve reviews for.
     *
     * @returns {Promise<Option<Review[]>>} - A promise that resolves to an optional array of Review objects.
     *       The array will be empty if no reviews are found.
     *       If an error occurs during the retrieval process, the promise will reject with the error.
     */
    get_reviews(id) {
        return __awaiter(this, void 0, void 0, function* () {
            const req = new RequestBuilder(`${this.config.host}/product/${id}/reviews`);
            req.set_method("GET");
            try {
                const res = yield req.send();
                return yield res.json();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
    /**
     * Retrieves the shopping cart from the server.
     *
     * @returns {Promise<Option<Product[]>>} A promise that resolves to an {@link Option} containing an array of {@link Product}s if successful,
     * or an error object if unsuccessful.
     */
    get_shopping_cart() {
        return __awaiter(this, void 0, void 0, function* () {
            const req = new RequestBuilder(`${this.config.host}/shopping_cart/fetch`);
            req.set_method("GET");
            try {
                const res = yield req.send();
                return yield res.json();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
    add_article_to_cart(id) {
        return __awaiter(this, void 0, void 0, function* () {
            const req = new RequestBuilder(`${this.config.host}/shopping_cart/articles/add`);
            req.set_method("POST");
            req.set_body(new URLSearchParams({
                product: id
            }));
            try {
                const res = yield req.send();
                return yield res.text();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
    remove_cart_article(id) {
        return __awaiter(this, void 0, void 0, function* () {
            const req = new RequestBuilder(`${this.config.host}/shopping_cart/articles/remove`);
            req.set_method("DELETE");
            req.set_body(new URLSearchParams({
                product: id
            }));
            //req.add_header("Content-Type:", "application/x-www-form-urlencoded")
            console.log(req);
            try {
                const res = yield req.send();
                return yield res.text();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
    clear_shopping_cart() {
        return __awaiter(this, void 0, void 0, function* () {
            const req = new RequestBuilder(`${this.config.host}/shopping_cart/clear`);
            req.set_method("GET");
            try {
                const res = yield req.send();
                return yield res.json();
            }
            catch (error) {
                console.error(error);
                return error;
            }
        });
    }
}
/**
 * RequestBuilder class is responsible for building and representing network requests.
 */
class RequestBuilder {
    /**
     * Creates a new instance of the constructor.
     *
     * @param {string} url - The URL to be assigned to the instance.
     */
    constructor(url) {
        this.url = url;
    }
    /**
     * Set the HTTP method for the request.
     *
     * @param {Method} method - The HTTP method to set.
     */
    set_method(method) {
        this.method = method;
    }
    /**
     * Sets the body of the object.
     *
     * @param {string} body - The body to be set.
     */
    set_body(body) {
        this.body = body;
    }
    add_header(key, value) {
        if (!this.headers)
            this.headers = {};
        this.headers[key] = value;
    }
    /**
     * Sends a request to the specified URL using the specified HTTP method.
     *
     * @return {Promise<Response>} A promise that resolves to a Response object representing the response to the request.
     */
    send() {
        let config = {
            method: this.method || "GET",
            body: this.body || null,
        };
        if (this.headers) {
            config.headers = this.headers;
        }
        return fetch(this.url, config);
    }
}
export { PedalizeApi, RequestBuilder };

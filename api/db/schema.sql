DROP TABLE IF EXISTS shopping_cart_article;
DROP TABLE IF EXISTS shopping_cart;
DROP TABLE IF EXISTS reviews;
DROP TABLE IF EXISTS product_characteristic;
DROP TABLE IF EXISTS product;
DROP TABLE IF EXISTS clients;

-- Les clients enregistrés
CREATE OR REPLACE TABLE clients (
    id CHAR(36) NOT NULL,
    token CHAR(64) NOT NULL,
    passwd CHAR(64) NOT NUll,
    username VARCHAR(32),

    PRIMARY KEY (id)
);

-- Contient les produits
CREATE OR REPLACE TABLE product (
    id CHAR(36),
    name VARCHAR(256) NOT NULL,
    description VARCHAR(2048),
    price INT UNSIGNED NOT NULL DEFAULT 1,

    main_image VARCHAR(256), -- Can be null :)
    second_image VARCHAR(256), -- Can be null :)
    third_image VARCHAR(256), -- Can be null :)
    fourth_image VARCHAR(256), -- Can be null :)

    size TINYINT NOT NULL DEFAULT 0,
    wheel_size TINYINT NOT NULL DEFAULT 0,

    UNIQUE (main_image, second_image, third_image, fourth_image),
    CHECK (price > 0),
    PRIMARY KEY (id)
);

-- Contiendra toutes les caractéristiques d'un produit
CREATE OR REPLACE TABLE product_characteristic (
    name VARCHAR(128),
    detail VARCHAR(256),
    product CHAR(36),

    FOREIGN KEY (product) REFERENCES product (id) ON DELETE CASCADE,
    PRIMARY KEY (product, name)
);

-- Contient les avis d'un produit
CREATE OR REPLACE TABLE reviews (
    id CHAR(36) NOT NULL,
    product CHAR(36) NOT NULL,
    user CHAR(36),
    review VARCHAR(512),
    stars TINYINT UNSIGNED NOT NULL DEFAULT 0,

    CHECK (0 <= stars <= 5),
    FOREIGN KEY (user) REFERENCES clients (id) ON DELETE CASCADE,
    FOREIGN KEY (product) REFERENCES product (id) ON DELETE CASCADE,
    PRIMARY KEY (id)
);


-- Contient les informations sur le panier d'un utilisateur
CREATE OR REPLACE TABLE shopping_cart (
    user CHAR(36) NOT NULL,
    id CHAR(36) NOT NUll,

    UNIQUE (user),
    PRIMARY KEY (id)
);

-- Contient les articles avec leur quantité des paniers
CREATE OR REPLACE TABLE shopping_cart_article (
    shopping_cart_id CHAR(36) NOT NULL,
    product CHAR(36) NOT NULL,
    quantity INT UNSIGNED DEFAULT 1,

    FOREIGN KEY (product) REFERENCES product (id) ON DELETE CASCADE,
    FOREIGN KEY (shopping_cart_id) REFERENCES shopping_cart (id) ON DELETE CASCADE,
    CHECK (quantity > 0),
    PRIMARY KEY (shopping_cart_id, product)
);









INSERT INTO clients VALUE (
    '251ef68e-6d10-4e08-a8e9-d4f42ad9fe36',
    'cd251870133e6dc175f6f616ea1a6e0dde6601dbde6e2ec173d1cc720cac7b58',
    '909104cdb5b06af2606ed4a197b07d09d5ef9a4aad97780c2fe48053bce2be52', -- "yeet"
    'Test'
);

INSERT INTO product (id, name, description, price) VALUE (
    '30dfd1d3-76bd-4c3b-b988-dd8235f7f238',
    'Kit anti-crevaison',
    'Kit anti-crevaison pour vélo',
    10
);
INSERT INTO product (id, name, description, price, main_image) VALUES
    ('d2c0b25c-e39a-43d4-8f06-ce5ed2d7919c', 'Casque de cyclisme', 'Casque léger et aérodynamique pour une protection optimale lors de vos sorties à vélo.', 79.99, 'casque_cyclisme.jpg'),
    ('dcfd55fd-5891-41a8-a5b4-9ee4ff00a576', 'Maillot de cyclisme', 'Maillot respirant en tissu technique pour une performance maximale sur la route.', 49.99, 'maillot_cyclisme.jpg'),
    ('7a555048-64ae-4089-82de-2d9db6034e19', 'Pneus de vélo de route', 'Pneus haute performance offrant une adhérence exceptionnelle sur les routes asphaltées.', 29.99, 'pneus_route.jpg'),
    ('3678c5c0-db86-4ef9-8bdd-c8b96affe2f7', 'Short de cyclisme', 'Short rembourré pour plus de confort pendant de longues balades à vélo.', 39.99, 'short_cyclisme.jpg'),
    ('19a13f5d-8530-4da4-a1b8-ef42279dcffb', 'Gants de cyclisme', 'Gants ergonomiques avec rembourrage pour une meilleure prise en main du guidon.', 19.99, 'gants_cyclisme.jpg'),
    ('749c5198-b498-4981-9e48-b95123a88775', 'Lunettes de soleil pour cyclistes', 'Lunettes polarisées pour protéger vos yeux des rayons UV pendant vos sorties en plein air.', 29.99, 'lunettes_cyclistes.jpg'),
    ('f0e22302-db5c-4277-b890-9291ca89abb3', 'Sac à dos de cyclisme', 'Sac à dos léger et compact pour transporter vos affaires essentielles pendant vos trajets à vélo.', 49.99, 'sac_a_dos_cyclisme.jpg'),
    ('b3343d89-d275-435f-be8e-0d69ae186f9f', 'Bidon d\'eau de cyclisme', 'Bidon de 750 ml avec une valve anti-fuite pour vous hydrater pendant vos sorties.', 9.99, 'bidon_cyclisme.jpg'),
    ('97274f94-849f-4e01-b706-0374d06e30dc', 'Cadenas de vélo', 'Cadenas en acier durable pour protéger votre vélo contre le vol.', 19.99, 'cadenas_velo.jpg'),
    ('32559cd1-cb95-4a69-bde1-66fd2b7e3212', 'Éclairage de vélo LED', 'Ensemble d\'éclairage avant et arrière pour assurer votre visibilité pendant vos sorties nocturnes.', 14.99, 'eclairage_velo.jpg');

INSERT INTO product_characteristic (product, name, detail) VALUE
    ('f0e22302-db5c-4277-b890-9291ca89abb3', 'Capacité', '40L');


# SELECT
#     reviews.id AS id,
#     reviews.product AS product,
#     reviews.user AS user,
#     reviews.review AS review,
#     reviews.stars AS stars,
#     c.username AS username
# FROM
#     reviews
# LEFT JOIN clients c on reviews.user = c.id
# WHERE product = ?;
DELETE FROM shopping_cart_article;
DELETE FROM reviews;
DELETE FROM product_characteristic;
DELETE FROM product;


INSERT INTO product (id, name, description, main_image) VALUE (
    '502f846e-0641-45c0-9244-42fac8f5bfda',
    'Casque Crossframe Pro',
    'Le tout nouveau casque Crossframe Pro est destiné aux amateurs de sensations fortes, de sprints endiablés, et de tout ce qui s\'en rapproche. Léger, polyvalent, et muni des technologies MIPS® et BOA®, ce casque se tient prêt pour toutes les rides dans lesquelles vous embarquerez.',
    'casque_vtt_1.png'
);


INSERT INTO product_characteristic (name, detail, product) VALUES (
        'Système de protection MIPS®',
        'Réduit le mouvement de rotation appliqué à la tête en absorbant et en redirigeant les forces susceptibles d\'endommager le cerveau',
        '502f846e-0641-45c0-9244-42fac8f5bfda'
    ), (
        'coque',
        'La coque Varizorb™ EPS à double densité optimise le niveau de protection en répartissant les forces d’impact sur une surface plus large',
        '502f846e-0641-45c0-9244-42fac8f5bfda'
    ), (
        'Boucle à pression',
        'Boucle à pression FIDLOCK pour un enfilage et un retrait facile même avec des gants',
        '502f846e-0641-45c0-9244-42fac8f5bfda'
    ), (
        'Doublure confort',
        'Doublure confort antimicrobienne Ionic+® détachable et lavable qui élimine les odeurs tout en évacuant la transpiration',
        '502f846e-0641-45c0-9244-42fac8f5bfda'
    ), (
        'Ventilation optimisée',
        'Ventilation optimisée grâce à une mousse EPS canalisée et moulée pour un refroidissement efficace',
        '502f846e-0641-45c0-9244-42fac8f5bfda'
    ), (
        'Système de fixation BOA®',
        'Système de fixation BOA® à micro-ajustement qui vous permet d\'effectuer des réglages en mouvement',
        '502f846e-0641-45c0-9244-42fac8f5bfda'
    ), (
        'Inserts d\'aération en TPU',
        'Inserts d\'aération en TPU pour garder vos lunettes de soleil en sécurité lorsque vous ne les utilisez pas',
        '502f846e-0641-45c0-9244-42fac8f5bfda'
);


SELECT * FROM product;

UPDATE product
SET
    main_image = NULL
WHERE
    id != '502f846e-0641-45c0-9244-42fac8f5bfda';

UPDATE product SET price = 188 WHERE id = '502f846e-0641-45c0-9244-42fac8f5bfda';

SELECT name, product.main_image FROM product;

INSERT INTO product_characteristic (name, detail, product) VALUES
                                                               ('Cadre Ransom Alloy SL',
                                                                'Cadre en alliage Ransom SL pour une construction légère et robuste.',

                                                                'db8b3416-7073-4bcc-9794-25af9db1d547'),

                                                               ('Fourche FOX 38 Grip Performance 170 mm',
                                                                'Fourche FOX 38 Grip Performance avec 170 mm de débattement pour une excellente absorption des chocs.',
                                                                'db8b3416-7073-4bcc-9794-25af9db1d547'),

                                                               ('FOX Nude T EVOL, système Tracloc, 170 mm',
                                                                'Amortisseur arrière FOX Nude T EVOL avec système Tracloc et 170 mm de débattement.',
                                                                'db8b3416-7073-4bcc-9794-25af9db1d547'),

                                                               ('SRAM NX Eagle, 12 vitesses',
                                                                'Transmission SRAM NX Eagle à 12 vitesses pour des performances de changement de vitesse exceptionnelles.',
                                                                'db8b3416-7073-4bcc-9794-25af9db1d547'),

                                                               ('Freins à disque Shimano 4 pistons',
                                                                'Freins à disque Shimano 4 pistons pour une puissante capacité de freinage.',
                                                                'db8b3416-7073-4bcc-9794-25af9db1d547'),

                                                               ('Pneus Maxxis EXO+',
                                                                'Pneus Maxxis EXO+ offrant une excellente adhérence et une protection supplémentaire contre les crevaisons.',
                                                                'db8b3416-7073-4bcc-9794-25af9db1d547');

INSERT INTO product_characteristic (name, detail, product) VALUES
                                                               ('Cadre carbone Genius HMX',
                                                                'Cadre en carbone Genius HMX pour une légèreté et une rigidité exceptionnelles.',
                                                                'c8311158-a5ae-446e-a941-521e7428495d'),

                                                               ('Fourche FOX 36 Float Factory 160 mm',
                                                                'Fourche FOX 36 Float Factory avec 160 mm de débattement pour une excellente absorption des chocs.',
                                                                'c8311158-a5ae-446e-a941-521e7428495d'),

                                                               ('FOX Nude 5T EVOL, Commande TwinLoc, 150 mm',
                                                                'Amortisseur arrière FOX Nude 5T EVOL avec commande TwinLoc et 150 mm de débattement.',
                                                                'c8311158-a5ae-446e-a941-521e7428495d'),

                                                               ('Transmission SRAM XX1 Eagle AXS / 12 vitesses',
                                                                'Transmission SRAM XX1 Eagle AXS à 12 vitesses pour des performances de changement de vitesse électronique exceptionnelles.',
                                                                'c8311158-a5ae-446e-a941-521e7428495d'),

                                                               ('Freins à disque Shimano XTR 4 pistons',
                                                                'Freins à disque Shimano XTR 4 pistons pour une puissante capacité de freinage.',
                                                                'c8311158-a5ae-446e-a941-521e7428495d'),

                                                               ('Pneus souples Maxxis',
                                                                'Pneus souples Maxxis pour une excellente adhérence sur tous les terrains.',
                                                                'c8311158-a5ae-446e-a941-521e7428495d');

-- Cadre
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Cadre',
     'Canyon Lux WC CF - Cadre XC léger en carbone tout-suspendu, doté de l\'ADN du modèle phare Lux World Cup CFR, vainqueur de championnats du monde. Dimension de l\'Axe: 12x148 mm, Matériau: Carbone (CF)',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Amortisseur
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Amortisseur',
     'FOX Float DPS Factory Remote',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Fourche suspendue
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Fourche suspendue',
     'Fox Factory 32 Step Cast Remote - Dimension de l\'Axe: 15x110 mm, Diametre Des Fourreaux: 32 mm, Débattement: 100 mm, Diamètre du tube de direction: 1 1/8" - 1,5", Poids: 1.48 kg',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Transmission
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Transmission',
     'Accumulateur: SRAM Powerpack',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Dérailleur arrière',
     'SRAM GX Eagle AXS Transmission - Levier de dérailleur standard: PF 92, Poids: 82 g',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Chaîne
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Chaîne',
     'SRAM GX Eagle Transmission',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Freins
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Frein à disque',
     'SRAM Level Silver Stealth 2 Piston - Nombre de Pistons: 2',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Disque de frein',
     'SRAM Centerline Rounded - Taille: 160 mm, Poids: 113 g',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Roues
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Roue',
     'DT Swiss XRC 1700 SPLINE - Dimension de l\'Axe: 12x148 mm, Fixation Du Disque: 6 trous, Poids: 881 g',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Pneu
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Pneu',
     'Maxxis Ikon Exo 2.35" - Largeur: 2.35", Poids: 740 g',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Axe traversant
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Axe traversant',
     'Canyon Quixle Thru Axle - Dimension de l\'Axe: 12x148 mm',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Cockpit
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Cockpit',
     'Canyon CP0008 XC-Cockpit - Cockpit en carbone léger et rigide conçu pour le cross-country en collaboration avec les coureurs professionnels de Canyon. Matériau: Carbone (CF), Poids: 328 g',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Poignées
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Poignées',
     'Ergon GA20',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Selle
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Selle',
     'Ergon SR10 Pro',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Tige de selle
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Tige de selle',
     'Canyon SP0061 Carbon Seatpost - Diamètre Du Collier: 30,9 mm, Matériau: Carbone (CF)',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Éclairage
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Éclairage',
     'Lot de réflecteurs',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

-- Accessoires
INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Pédales',
     'Non inclus',
     'f0e22302-db5c-4277-b890-9291ca89abb3');

INSERT INTO product_characteristic (name, detail, product) VALUES
    ('Manuels et accessoires',
     'Boîte à outils Canyon',
     'f0e22302-db5c-4277-b890-9291ca89abb3');
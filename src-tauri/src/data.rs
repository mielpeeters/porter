pub fn colordrid() -> String {
    "<div class=\"color\" style=\"background-color: COLOR\"></div>".to_string()
}

pub fn image() -> String {
    "<img src=\"PATH\" alt=\"DESCR\" srcset=\"320W 320w, 640W 640w, 960W 960w, 1290W 1290w, 1920W 1920w, 2560W 2560w\" sizes=\"90vw\">".to_string()
}

pub fn item() -> String {
    "<div class=\"item\">
        <div class=\"bar clickable\">
        <div class=\"date\">
        <h2><span class=\"look\">YEAR</span></h2>
        </div>
        <div class=\"name\">
        <h2><span class=\"look\">NAME</span></h2>
        </div>
        <div class=\"measurements\">
        <h2><span class=\"look\">MEASUREMENTS</span></h2>
        </div>
        <div class=\"colorgrid\">
        COLORGRID
        </div>
        </div>
        <div class=\"content\">
        <div class=\"scrollButton scrollRight\"></div>
        <div class=\"scrollButton scrollLeft\"></div>
        <div class=\"scrollmenu\">
        IMAGES
        <div class=\"projecttext\">
        DESCRIPTION
        </div>
        </div>
        </div>
        </div>
        "
    .to_string()
}

pub fn page() -> String {
    r####"<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8" />
    <meta name="keywords" content="Kaat Neyrinck, Graphic Designer, Antwerp, Portfolio">
    <meta name="description"
        content="Portfolio of graphic designer Kaat Neyrinck, based in Antwerp. Her style is simple and effective.">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="theme-color" content="#eaeaea">
    <title>Kaat Neyrinck</title>
    <link rel="stylesheet" type="text/css" href="css/main.css" />
    <link rel="icon" type="image/png" href="/images/favicon/favicon_16.png" sizes="16x16">
    <link rel="icon" type="image/png" href="/images/favicon/favicon_32.png" sizes="32x32">
    <link rel="shortcut icon" type="image/png" href="/images/favicon/favicon_96.png" sizes="96x96">
    <link rel="icon" type="image/png" href="/images/favicon/favicon_180.png" sizes="180x180">
    <link rel="icon" type="image/png" href="/images/favicon/favicon_192.png" sizes="192x192">
    <link rel="icon" type="image/png" href="/images/favicon/favicon_512.png" sizes="512x512">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    
    <link rel="preload" href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap" as="style" onload="this.onload=null;this.rel='stylesheet'">
    <noscript><link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap" rel="stylesheet"></noscript>  

    <script type="application/ld+json">
        {
        "@context": "https://schema.org",
        "@type": "Organization",
        "url": "https://kaatneyrinck.com",
        "logo": "https://kaatneyrinck.com/images/logo/logo%2025x25-02.png"
        }
    </script>
</head>


<body>
    <div id="cursor">
        <svg viewBox="-110 -110 220 220" preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg">
            <ellipse shape-rendering="geometricPrecision" ry="90" rx="90" id="eyeLid" cy="0" cx="0" stroke="#000" fill="#ffffff" stroke-width="20" />
            <ellipse shape-rendering="geometricPrecision" ry="40" rx="40" id="eyeBall" cy="0" cx="30" stroke="#000" fill="#000000" />
        </svg>
    </div>

    <div id="quickFix">
        <div class="container">
            <div class="item headerItem">
                <div class="bar header">
                    <div class="Kaat">
                        <h1><span class="look">Kaat Neyrinck</span></h1>
                    </div>
                    <button class="openclose">
                        <div class="name-inner">
                            <h1><span class="look">open</span></h1>
                        </div>
                    </button>
                </div>
            </div>
            <div class="about" id="about">
                <div class="abouttext">
                    <title>ABOUT</title>
                    <p>Kaat Neyrinck (Mortsel, 2003°) is an Antwerp based graphic design student. In 2023 she will finish her
                        bachelor degree in graphic design at Sint-Lucas Antwerp.
                        Her work is toughtful, refined, colorful and playful. The main focus is on colors and typography. Good
                        graphic design is telling the whole story in the most refined way.</p>
                </div>
            </div>
       
            ITEMS

        </div>
        <div class="bar-info">
            <div class="bar clickable">
                <div class="info look">
                    <a href="mailto:kaat.neyrinck@gmail.com"><h2>kaat.neyrinck@gmail.com</h2></a>
                </div>
            </div>
            <div class="bar" style="border-top: 0;border-bottom:0">
                <div class="info">
                    <a href="tel:0032470325296"><h2>+32(0)470325296</h2></a>
                </div>
            </div>
            <div class="bar clickable">
                <div class="info">
                    <a class="link look" href="https://www.instagram.com/kaat.neyrinck/">
                        <h2 style="text-decoration: underline;">Instagram</h2>
                    </a>
                </div>
            </div>
        </div>
    </div>


    <div class="end"></div>

    <script src="src/init.js"></script>
    <script src="src/folding.js"></script>
    <script src="src/eyeCursor.js"></script>
</body>

</html>"####.to_string()
}

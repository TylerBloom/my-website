
use std::fs;

static BASE_HTML_START: &str = "
<!DOCTYPE html>
<html>
    <head>
        <title>Tyler's Blog</title>
    </head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    <link rel=\"stylesheet\" href=\"https://www.w3schools.com/w3css/4/w3.css\">
    <link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css?family=Montserrat\">
    <link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css\">
    <style>
        body, h1,h2,h3,h4,h5,h6 {font-family: \"Montserrat\", sans-serif}
        .w3-row-padding img {margin-bottom: 12px}
        .w3-sidebar {width: 120px;background: #222;}
        #main {margin-left: 120px}
        @media only screen and (max-width: 600px) {#main {margin-left: 0}}
    </style>
    <body class=\"w3-black\">
        <!-- Icon Bar (Sidebar - hidden on small screens) -->
        <nav class=\"w3-sidebar w3-bar-block w3-small w3-hide-small w3-center\">
        <!-- Avatar image in top left corner -->
        <img src=\"/static/avatar.png\" style=\"width:100%\">
        <a href=\"/static/index.html\" class=\"w3-bar-item w3-button w3-padding-large w3-black\">
            <i class=\"fa fa-home w3-xxlarge\"></i>
            <p>HOME</p>
        </a>
        <a href=\"/static/bio.html\" class=\"w3-bar-item w3-button w3-padding-large w3-hover-black\">
            <i class=\"fa fa-user w3-xxlarge\"></i>
            <p>ABOUT</p>
        </a>
        <a href=\"/projects\" class=\"w3-bar-item w3-button w3-padding-large w3-hover-black\">
            <i class=\"fa fa-eye w3-xxlarge\"></i>
            <p>PROJECTS</p>
        </a>
        <a href=\"/blog\" class=\"w3-bar-item w3-button w3-padding-large w3-hover-black\">
            <i class=\"fa fa-envelope w3-xxlarge\"></i>
            <p>BLOG</p>
        </a>
        </nav>

        <!-- Navbar on small screens (Hidden on medium and large screens) -->
        <div class=\"w3-top w3-hide-large w3-hide-medium\" id=\"myNavbar\">
        <div class=\"w3-bar w3-black w3-opacity w3-hover-opacity-off w3-center w3-small\">
            <a href=\"/static/index.html\" class=\"w3-bar-item w3-button\" style=\"width:25% !important\">HOME</a>
            <a href=\"/static/bio.html\" class=\"w3-bar-item w3-button\" style=\"width:25% !important\">ABOUT</a>
            <a href=\"/projects\" class=\"w3-bar-item w3-button\" style=\"width:25% !important\">PROJECTS</a>
            <a href=\"/blog\" class=\"w3-bar-item w3-button\" style=\"width:25% !important\">BLOG</a>
        </div>
        </div>


    <div class=\"w3-padding-large\" id=\"main\">
";

static BASE_HTML_END: &str = "
    </div>
  </body>
</html>
";

pub fn construct_html_from_base( body: String ) -> String {
    let mut digest: String = String::from(BASE_HTML_START);
    digest += &body;
    digest += BASE_HTML_END;
    digest
}

pub struct IconGrid {
    pub icons: Vec<String>,
}

impl IconGrid {
    pub fn load( files: Vec<String> ) -> Self {
        let mut icons: Vec<String> = Vec::new();
        for f in files {
            match fs::read_to_string(f) {
                Err(..) => {},
                Ok(icon) => { icons.push(icon); },
            }
        }
        IconGrid { icons }
    }
    
    pub fn export_to_grid( &self ) -> String {
        let grid_start = "<div class=\"w3-row-padding\">";
        let grid_close = "</div>";
        let mut digest: String = String::from(grid_start);
        for (i, icon) in self.icons.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                digest += grid_close;
                digest += "\n";
                digest += grid_start;
                digest += "\n";
            }
            digest += &icon;
            digest += "\n";
        }
        digest += grid_close;
        digest
    }
}

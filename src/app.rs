
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::fs;
use serde_yaml; 
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct PostMeta {
    descripcion: String,
    titulo: String,
    fecha: String,
    archivo: String
}

#[derive(Params, PartialEq)]
struct PostParams {
    archivo: Option<String>
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        <Link
            rel="stylesheet"
            href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/atom-one-dark.css"
        />
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/rust.min.js"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/go.min.js"/>
        <Stylesheet id="leptos" href="pkg/blog.css"/>
        // sets the document title
        <Title text="Blog de Tomás"/>
        // content for this welcome page

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Layout>
                <Routes>
                    <Route path="" view=BlogHomePage/>
                    <Route path="/secret" view=|| view! { <p>secreto</p> }/>
                    <Route path="/post/:archivo" view=Post/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/projects" view=ProjectsPage/>
                </Routes>
            </Layout>
        </Router>
    }
}


#[server(GetPost)]
async fn get_post(name: String) -> Result<String, ServerFnError> {
    let file_content = fs::read_to_string(format!("posts/{}/{}", name.split(".").collect::<Vec<&str>>()[0], name)).unwrap();
    let post = markdown::to_html(&file_content);
    Ok(post)
}

#[server(GetPostsMeta)]
 async fn get_posts_meta() -> Result<Vec<PostMeta>, ServerFnError> {
    let paths = fs::read_dir("posts").unwrap();
    let mut posts_meta: Vec<PostMeta> = Vec::new();
   
    for p in paths {
      
        let pval = p.unwrap();
       
        let post_dirs = fs::read_dir(pval.path()).unwrap();
        
        for dir in post_dirs {
            
            let file_path = format!("{}", dir?.path().display());

                if file_path.ends_with("yml") {

                    let file_content = fs::read_to_string(file_path).unwrap();

                    let meta = serde_yaml::from_str(&file_content).unwrap();

                    posts_meta.push(meta);

                }
        }
    }

    Ok(posts_meta)

}


#[component]
fn BlogPost(title: String, content: String, date: String, link: String) -> impl IntoView {
    view! {
        <article class="bg-white shadow-md rounded-lg overflow-hidden mb-8">
            <div class="p-6">
                <h2 class="text-2xl font-bold mb-2">{title.clone()}</h2>
                <p class="text-gray-600 mb-4">{date}</p>
                <p class="text-gray-700 mb-4">{content}</p>
                <a href=link class="text-blue-600 hover:text-blue-800 font-semibold">
                    Leer más
                </a>
            </div>
        </article>
    }
}

#[component]
fn Post() -> impl IntoView {
    let params = use_params_map();

    let archivo = move || {
        params.with(|params| params.get("archivo").cloned())
            .unwrap_or_default()
    };

    let post = create_resource(
        move || archivo(),
        |archivo| async move { get_post(archivo).await }
    );

    view! {
        <Suspense fallback=move || {
            view! { <p>"Cargando posts..."</p> }
        }>

            {move || match post.get() {
                Some(Ok(content)) => {
                    view! { <div class="blog-post" inner_html=content></div> }
                }
                Some(Err(_)) => view! { <div>"Error cargando post"</div> },
                None => view! { <div>"Cargando..."</div> },
            }}
            <script>hljs.highlightAll();</script>
        </Suspense>
    }
}

#[component]
fn BlogHomePage() -> impl IntoView {
    let posts = create_resource(
        || (),
        |_| async move { get_posts_meta().await }
    );

    view! {
        <Suspense fallback=move || {
            view! { <p>"Cargando posts..."</p> }
        }>
            {move || {
                posts
                    .get()
                    .map(|result| {
                        result
                            .map(|v| {
                                v.into_iter()
                                    .rev()
                                    .map(|n| {
                                        view! {
                                            <BlogPost
                                                title=n.titulo
                                                content=n.descripcion
                                                date=n.fecha
                                                link=format!("/post/{}", n.archivo)
                                            />
                                        }
                                    })
                                    .collect_view()
                            })
                            .unwrap_or_else(|_| ().into_view())
                    })
            }}

        </Suspense>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold mb-6 text-gray-800">"Acerca"</h1>

            <div class="bg-white shadow-md rounded-lg p-6 mb-8">
                <h2 class="text-2xl font-semibold mb-4 text-gray-700">"¿Quién soy?"</h2>
                <p class="text-gray-600 mb-4">
                    "¡Hola! Soy Tomás Varas. Creé este blog para compartir mis experiencias en el desarrollo de software."
                </p>
            </div>

            <div class="bg-white shadow-md rounded-lg p-6 mb-8">
                <h2 class="text-2xl font-semibold mb-4 text-gray-700">"Mis habilidades"</h2>
                <ul class="list-disc pl-6 text-gray-600">
                    <li>"Desarrollo móvil"</li>
                    <li>"Desarrollo Web (front & back)"</li>
                </ul>
            </div>

            <div class="bg-white shadow-md rounded-lg p-6 mb-8">
                <h2 class="text-2xl font-semibold mb-4 text-gray-700">"Propósito del blog"</h2>
                <p class="text-gray-600 mb-4">"Esta es una plataforma para:"</p>
                <ul class="list-disc pl-6 text-gray-600">
                    <li>"Compartir conocimiento y lecciones de mis experiencias programando"</li>
                    <li>"Explorar y explicar conceptos de programación"</li>
                    <li>"Documentar mi aprendizaje y compartir mis proyectos"</li>
                </ul>
            </div>

            <div class="bg-white shadow-md rounded-lg p-6">
                <h2 class="text-2xl font-semibold mb-4 text-gray-700">"Contacto"</h2>
                <p class="text-gray-600 mb-4">
                    "Siempre estoy dispuesto a conectar, me podés hablar vía:"
                </p>
                <ul class="list-disc pl-6 text-gray-600">
                    <li>"Email: tomas_varas1@outlook.com"</li>
                    <li>
                        "GitHub: "
                        <a
                            href="https://github.com/obskur123"
                            class="text-blue-600 hover:underline"
                        >
                            "@obskur123"
                        </a>
                    </li>
                </ul>
            </div>
        </div>
    }
}


#[component]
fn ProjectsPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold mb-6 text-gray-800">"Mis Proyectos"</h1>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                // Proyecto 1
                <div class="bg-white shadow-md rounded-lg overflow-hidden">
                    <div class="p-6">
                        <h2 class="text-2xl font-semibold mb-2 text-gray-700">"Blog Personal"</h2>
                        <p class="text-gray-600 mb-4">
                            "Este mismo blog lo construí con Rust y Leptos, 
                            con una idea similar a un generador de páginas estáticas,
                            cada post es un archivo markdown que se parsea a HTML."
                        </p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span class="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Rust"
                            </span>
                            <span class="bg-green-100 text-green-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Leptos"
                            </span>
                            <span class="bg-yellow-100 text-yellow-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Desarrollo Web"
                            </span>
                        </div>
                        <a
                            href="https://github.com/obskur123/blog"
                            class="text-blue-600 hover:underline"
                            target="_blank"
                        >
                            "Ver en GitHub"
                        </a>
                    </div>
                </div>

                // Proyecto 2
                <div class="bg-white shadow-md rounded-lg overflow-hidden">
                    <div class="p-6">
                        <h2 class="text-2xl font-semibold mb-2 text-gray-700">"Clon de Snake"</h2>
                        <p class="text-gray-600 mb-4">
                            "Un clon del juego Snake desarrollado con Rust y Macroquad, compilado a WASM."
                        </p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span class="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Rust"
                            </span>
                            <span class="bg-green-100 text-green-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Macroquad"
                            </span>
                            <span class="bg-yellow-100 text-yellow-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "WASM"
                            </span>
                        </div>
                        <ul>
                            <li>
                                <a
                                    href="https://github.com/obskur123/snake-rs"
                                    class="text-blue-600 hover:underline"
                                    target="_blank"
                                >
                                    "Ver en GitHub"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://obskur123.github.io/snake-rs/"
                                    class="text-blue-600 hover:underline"
                                    target="_blank"
                                >
                                    "Jugar"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>

                // Proyecto 3
                <div class="bg-white shadow-md rounded-lg overflow-hidden">
                    <div class="p-6">
                        <h2 class="text-2xl font-semibold mb-2 text-gray-700">"Clon de Tetris"</h2>
                        <p class="text-gray-600 mb-4">
                            "Un clon de Tetris desarrollado con Rust y Macroquad, compilado a WASM."
                        </p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span class="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Rust"
                            </span>
                            <span class="bg-green-100 text-green-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Macroquad"
                            </span>
                            <span class="bg-yellow-100 text-yellow-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "WASM"
                            </span>
                        </div>
                        <ul>
                            <li>
                                <a
                                    href="https://github.com/obskur123/tetris-rs"
                                    class="text-blue-600 hover:underline"
                                    target="_blank"
                                >
                                    "Ver en GitHub"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://obskur123.github.io/tetris-rs/"
                                    class="text-blue-600 hover:underline"
                                    target="_blank"
                                >
                                    "Jugar"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>

                // Proyecto 4
                <div class="bg-white shadow-md rounded-lg overflow-hidden">
                    <div class="p-6">
                        <h2 class="text-2xl font-semibold mb-2 text-gray-700">
                            "Calculadora/Intérprete"
                        </h2>
                        <p class="text-gray-600 mb-4">
                            "Un pequeño intérprete de expresiones matemáticas desarrollado con Vue 3 y TypeScript."
                        </p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span class="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Vue 3"
                            </span>
                            <span class="bg-green-100 text-green-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "TypeScript"
                            </span>
                            <span class="bg-yellow-100 text-yellow-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "JavaScript"
                            </span>
                        </div>
                        <ul>
                            <li>
                                <a
                                    href="https://github.com/obskur123/calculator"
                                    class="text-blue-600 hover:underline"
                                    target="_blank"
                                >
                                    "Ver en GitHub"
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://github.com/obskur123/calculator"
                                    class="text-blue-600 hover:underline"
                                    target="_blank"
                                >
                                    "Link"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}


#[component]
fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col bg-gray-100 min-h-screen">
            <header class="bg-white shadow-md">
                <div class="container mx-auto px-4 py-6">
                    <h1 class="text-3xl font-bold text-gray-800">"Tomás Varas Blog"</h1>
                    <nav class="mt-4">
                        <a href="/" class="text-gray-600 hover:text-gray-800 mr-4">
                            "Inicio"
                        </a>
                        <a href="/about" class="text-gray-600 hover:text-gray-800 mr-4">
                            "Acerca"
                        </a>
                        <a href="/projects" class="text-gray-600 hover:text-gray-800">
                            "Proyectos"
                        </a>
                    </nav>
                </div>
            </header>
            <main class="flex-grow container mx-auto px-4 py-8 blog-post">{children()}</main>
            <footer class="bg-gray-800 text-white">
                <div class="container mx-auto px-4 py-6 text-center">
                    <p>"<2024 Tomás Varas/>"</p>
                </div>
            </footer>
        </div>
    }
}
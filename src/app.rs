
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
            view! { <p>"Loading posts..."</p> }
        }>

            {move || match post.get() {
                Some(Ok(content)) => {
                    view! { <div class="blog-post" inner_html=content></div> }
                }
                Some(Err(_)) => view! { <div>"Error loading post"</div> },
                None => view! { <div>"Loading..."</div> },
            }}
            <script>
                hljs.highlightAll();
            </script>
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
            view! { <p>"Loading posts..."</p> }
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
            <h1 class="text-4xl font-bold mb-6 text-gray-800">"About Me"</h1>

            <div class="bg-white shadow-md rounded-lg p-6 mb-8">
                <h2 class="text-2xl font-semibold mb-4 text-gray-700">"Who am I?"</h2>
                <p class="text-gray-600 mb-4">
                    "Hello! I'm Tomas Varas, a passionate developer and tech enthusiast. 
                    I created this blog to share my journey in the world of programming, 
                    with a focus on Rust, web development, and emerging technologies."
                </p>
            </div>

            <div class="bg-white shadow-md rounded-lg p-6 mb-8">
                <h2 class="text-2xl font-semibold mb-4 text-gray-700">"My Expertise"</h2>
                <ul class="list-disc pl-6 text-gray-600">
                    <li>"Rust programming"</li>
                    <li>"Web development (Frontend and Backend)"</li>
                    <li>"System design and architecture"</li>
                    <li>"Open source contributions"</li>
                </ul>
            </div>

            <div class="bg-white shadow-md rounded-lg p-6 mb-8">
                <h2 class="text-2xl font-semibold mb-4 text-gray-700">"Blog's Purpose"</h2>
                <p class="text-gray-600 mb-4">"This blog serves as a platform for me to:"</p>
                <ul class="list-disc pl-6 text-gray-600">
                    <li>"Share insights and lessons from my coding experiences"</li>
                    <li>"Explore and explain new technologies and programming concepts"</li>
                    <li>"Connect with like-minded developers and enthusiasts"</li>
                    <li>"Document my learning journey and personal projects"</li>
                </ul>
            </div>

            <div class="bg-white shadow-md rounded-lg p-6">
                <h2 class="text-2xl font-semibold mb-4 text-gray-700">"Get in Touch"</h2>
                <p class="text-gray-600 mb-4">
                    "I'm always excited to connect with fellow developers and tech enthusiasts. 
                    Feel free to reach out to me through:"
                </p>
                <ul class="list-disc pl-6 text-gray-600">
                    <li>"Email: tomas@example.com"</li>
                    <li>
                        "GitHub: "
                        <a
                            href="https://github.com/tomasvaras"
                            class="text-blue-600 hover:underline"
                        >
                            "@tomasvaras"
                        </a>
                    </li>
                    <li>
                        "Twitter: "
                        <a
                            href="https://twitter.com/tomasvaras"
                            class="text-blue-600 hover:underline"
                        >
                            "@tomasvaras"
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
            <h1 class="text-4xl font-bold mb-6 text-gray-800">"My Projects"</h1>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                // Project 1
                <div class="bg-white shadow-md rounded-lg overflow-hidden">
                    <div class="p-6">
                        <h2 class="text-2xl font-semibold mb-2 text-gray-700">
                            "Rust Blog Engine"
                        </h2>
                        <p class="text-gray-600 mb-4">
                            "A high-performance blog engine built with Rust and Leptos, 
                            featuring server-side rendering and dynamic content loading."
                        </p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span class="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Rust"
                            </span>
                            <span class="bg-green-100 text-green-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Leptos"
                            </span>
                            <span class="bg-yellow-100 text-yellow-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Web Development"
                            </span>
                        </div>
                        <a
                            href="https://github.com/yourusername/rust-blog-engine"
                            class="text-blue-600 hover:underline"
                        >
                            "View on GitHub"
                        </a>
                    </div>
                </div>

                // Project 2
                <div class="bg-white shadow-md rounded-lg overflow-hidden">
                    <div class="p-6">
                        <h2 class="text-2xl font-semibold mb-2 text-gray-700">
                            "Go Microservices Framework"
                        </h2>
                        <p class="text-gray-600 mb-4">
                            "A lightweight microservices framework in Go, designed for 
                            building scalable and maintainable distributed systems."
                        </p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span class="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Go"
                            </span>
                            <span class="bg-green-100 text-green-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Microservices"
                            </span>
                            <span class="bg-yellow-100 text-yellow-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Backend"
                            </span>
                        </div>
                        <a
                            href="https://github.com/yourusername/go-micro-framework"
                            class="text-blue-600 hover:underline"
                        >
                            "View on GitHub"
                        </a>
                    </div>
                </div>

                // Project 3
                <div class="bg-white shadow-md rounded-lg overflow-hidden">
                    <div class="p-6">
                        <h2 class="text-2xl font-semibold mb-2 text-gray-700">
                            "AI-Powered Code Reviewer"
                        </h2>
                        <p class="text-gray-600 mb-4">
                            "An AI-driven tool that automatically reviews code, suggests improvements, 
                            and detects potential bugs using machine learning algorithms."
                        </p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span class="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Python"
                            </span>
                            <span class="bg-green-100 text-green-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Machine Learning"
                            </span>
                            <span class="bg-yellow-100 text-yellow-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "DevOps"
                            </span>
                        </div>
                        <a
                            href="https://github.com/yourusername/ai-code-reviewer"
                            class="text-blue-600 hover:underline"
                        >
                            "View on GitHub"
                        </a>
                    </div>
                </div>

                // Project 4
                <div class="bg-white shadow-md rounded-lg overflow-hidden">
                    <div class="p-6">
                        <h2 class="text-2xl font-semibold mb-2 text-gray-700">
                            "Crypto Portfolio Tracker"
                        </h2>
                        <p class="text-gray-600 mb-4">
                            "A web application for tracking cryptocurrency portfolios, 
                            featuring real-time price updates and performance analytics."
                        </p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span class="bg-blue-100 text-blue-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "JavaScript"
                            </span>
                            <span class="bg-green-100 text-green-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "React"
                            </span>
                            <span class="bg-yellow-100 text-yellow-800 text-sm font-medium px-2.5 py-0.5 rounded">
                                "Blockchain"
                            </span>
                        </div>
                        <a
                            href="https://github.com/yourusername/crypto-portfolio-tracker"
                            class="text-blue-600 hover:underline"
                        >
                            "View on GitHub"
                        </a>
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
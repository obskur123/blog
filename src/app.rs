use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="pkg/blog.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=BlogHomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=on_click class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "Click number " {count}
                    </button>
                </div>
            </div>
        </main>
    }
}

#[component]
fn BlogPost(title: String, content: String, date: String) -> impl IntoView {
    view! {
        <article class="bg-white shadow-md rounded-lg overflow-hidden mb-8">
            <div class="p-6">
                <h2 class="text-2xl font-bold mb-2">{title}</h2>
                <p class="text-gray-600 mb-4">{date}</p>
                <p class="text-gray-700 mb-4">{content}</p>
                <a href="#" class="text-blue-600 hover:text-blue-800 font-semibold">Read more</a>
            </div>
        </article>
    }
}

#[component]
fn BlogHomePage() -> impl IntoView {
    view! { 
        <div class="bg-gray-100 min-h-screen">
            <header class="bg-white shadow-md">
                <div class="container mx-auto px-4 py-6">
                    <h1 class="text-3xl font-bold text-gray-800">"My Tech Blog"</h1>
                    <nav class="mt-4">
                        <a href="#" class="text-gray-600 hover:text-gray-800 mr-4">"Home"</a>
                        <a href="#" class="text-gray-600 hover:text-gray-800 mr-4">"About"</a>
                        <a href="#" class="text-gray-600 hover:text-gray-800">"Contact"</a>
                    </nav>
                </div>
            </header>

            <main class="container mx-auto px-4 py-8">
                <BlogPost
                    title="Getting Started with Rust".to_string()
                    content="Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety...".to_string()
                    date="July 11, 2024".to_string()
                />
                <BlogPost
                    title="Web Development with Leptos".to_string()
                    content="Leptos is a full-stack framework for building web applications with Rust. It offers a reactive approach to UI...".to_string()
                    date="July 8, 2024".to_string()
                />
                <BlogPost
                    title="The Power of Tailwind CSS".to_string()
                    content="Tailwind CSS is a utility-first CSS framework that can speed up your development process significantly...".to_string()
                    date="July 5, 2024".to_string()
                />
            </main>

            <footer class="bg-gray-800 text-white">
                <div class="container mx-auto px-4 py-6 text-center">
                    <p>"© 2024 My Tech Blog. All rights reserved."</p>
                </div>
            </footer>
        </div>
    }
}


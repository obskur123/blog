# Blog en Rust: Introducción

Rust es un lenguaje de programación de sistemas que se enfoca en la seguridad y velocidad de ejecución. Posee una poderosa biblioteca estándar y una gran comunidad (a veces demasiado grande).

Gracias a la introducción de WebAssembly en los navegadores hace ya algunos años, se abrió una gran puerta en el mundo del desarrollo web. La capacidad de usar WebAssembly (WASM) como target de compilación permite utilizar otros lenguajes aparte de JavaScript en el cliente.

Esto habilita a los desarrolladores a portar y desarrollar para la web en su lenguaje de preferencia, pudiendo utilizar librerías de otros lenguajes desde JavaScript y acceder a interoperabilidad entre lenguajes sin la necesidad de FFI.

Por los esfuerzos de la comunidad de Rust, es posible crear interfaces súper veloces, seguras y fuertemente tipadas. Contamos con todos los accesorios que podrías encontrar en un framework convencional de JavaScript, como: React, Vue y Angu... **cof, cof** Svelte.

En este proyecto decidí utilizar Leptos, un framework de front inspirado en React con baterías incluidas. Cuenta con un router, algunas formas de manejar estado, capacidad de SSR entre otros detalles.

Antes de empezar, debo mencionar que el blog está desarrollado utilizando la versión SSR (Server Side Rendering) de Leptos, es decir que los componentes se ejecutan en el servidor y luego son hidratados, un concepto que se utiliza en estos escenarios para agregar interactividad a nuestros componentes renderizados en el servidor.

## Otras dependencias e ideas

Mi idea era hacer algo como una página generada estáticamente, crear entradas de posteos a partir de archivos de Markdown, así que empecé a buscar algún crate que me ayudara con esto. Buscando un poco me encontré con **[markdown-rs](https://github.com/wooorm/markdown-rs)** que hace muy sencillo parsear Markdown a HTML.

También creé un archivo YAML que acompaña cada Markdown agregando un poco de metadata a cada posteo como el título, descripción y la fecha. Estos archivos también necesitan ser parseados, así que me ayudé con **[serde-yaml](https://github.com/dtolnay/serde-yaml)**.

Entonces... Cada posteo sería un directorio con un archivo Markdown con el contenido y otro archivo YAML con su metadata.

Finalmente, usando las server functions de Leptos podría fetchearlos y entregarlos al cliente.

## Resumen de la implementación

Primero definí un struct con los datos que tendrá la metadata de cada posteo.

```rust
pub struct PostMeta {
    descripcion: String,
    titulo: String,
    fecha: String,
    archivo: String
}
```

Necesito mostrar cada entrada en el inicio, así que voy a buscar todas.

```rust
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
```

Para mostrar los contenidos asumo que cada directorio se llama igual que el archivo de Markdown, para hacerme más sencilla la vida.

```rust
#[server(GetPost)]
async fn get_post(name: String) -> Result<String, ServerFnError> {
    let file_content = fs::read_to_string(format!("posts/{}/{}", name.split(".").collect::<Vec<&str>>()[0], name)).unwrap();
    let post = markdown::to_html(&file_content);
    Ok(post)
}
```

Como estamos buscando los datos asincrónicamente, nuestro componente necesita ser hidratado, así que por eso utilizamos ```<Suspense />``` que muestra un componente fallback hasta que nuestros recursos sean cargados o den error. ```create_resource``` tiene la ventaja de que se ejecuta en el servidor.

```rust
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
```

Finalmente podemos mostrar los contenidos de esta manera. Definí una ruta que recibe el nombre de un archivo y en el componente va a buscarlo.

```rust
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
```

## Conclusión

Fue una buena experiencia de aprendizaje elaborar este blog, me enseñó mucho del framework y el lenguaje.
Con un poco más de trabajo sería posible crear una capa encima de Leptos exclusivamente para generar blogs.


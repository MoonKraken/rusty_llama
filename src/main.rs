#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use rusty_llama::app::*;

    // debug logging, disable for prod
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let model = web::Data::new(load_language_model().await);

    println!("listening on http://{addr}");

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        App::new()
            .app_data(model.clone())
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(favicon)
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en" class="h-full">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body class="h-dvh overflow-hidden bg-zinc-900 font-sans text-white antialiased">
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

/// Public GGUF quant of Qwen3.8 27B. About 17 GB. mistral.rs downloads it
/// into the Hugging Face cache the first time the server starts.
#[cfg(feature = "ssr")]
const DEFAULT_MODEL_REPO: &str = "bartowski/Qwen3.8-27B-GGUF";
#[cfg(feature = "ssr")]
const DEFAULT_MODEL_FILE: &str = "Qwen3.8-27B-Q4_K_M.gguf";

#[cfg(feature = "ssr")]
async fn load_language_model() -> mistralrs::Model {
    use mistralrs::GgufModelBuilder;

    dotenv::dotenv().ok();
    let (model_id, files, label) = model_source();
    log::info!("loading {label}");

    let mut builder = GgufModelBuilder::new(model_id, files).with_logging();
    if let Ok(template) = std::env::var("CHAT_TEMPLATE") {
        if !template.is_empty() {
            builder = builder.with_chat_template(template);
        }
    }

    builder
        .build()
        .await
        .unwrap_or_else(|err| panic!("Failed to load {label}: {err}"))
}

/// `MODEL_PATH` loads a local `.gguf` file or a directory of shards.
/// Otherwise `MODEL_REPO` and `MODEL_FILE` name a Hugging Face repo, which
/// mistral.rs downloads when the file is not already in the HF cache.
#[cfg(feature = "ssr")]
fn model_source() -> (String, Vec<String>, String) {
    if let Ok(model_path) = std::env::var("MODEL_PATH") {
        if !model_path.trim().is_empty() {
            let path = std::path::PathBuf::from(model_path.trim());
            let (model_id, files) = local_gguf(&path);
            return (model_id, files, format!("local GGUF {}", path.display()));
        }
    }

    let repo = env_or("MODEL_REPO", DEFAULT_MODEL_REPO);
    let file = env_or("MODEL_FILE", DEFAULT_MODEL_FILE);
    let label = format!("{file} from Hugging Face repo {repo}");
    (repo, vec![file], label)
}

#[cfg(feature = "ssr")]
fn env_or(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => default.to_string(),
    }
}

/// `MODEL_PATH` may be one `.gguf` file or a directory of GGUF shards.
#[cfg(feature = "ssr")]
fn local_gguf(path: &std::path::Path) -> (String, Vec<String>) {
    use std::path::Path;

    if path.is_file() {
        let file_name = path
            .file_name()
            .expect("model path should include a file name")
            .to_string_lossy()
            .into_owned();
        let directory = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .to_string_lossy()
            .into_owned();
        return (directory, vec![file_name]);
    }

    if path.is_dir() {
        let mut files: Vec<String> = std::fs::read_dir(path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".gguf"))
            .collect();
        files.sort();
        if files.is_empty() {
            panic!("no .gguf files found in {}", path.display());
        }
        return (path.to_string_lossy().into_owned(), files);
    }

    panic!(
        "MODEL_PATH does not exist: {}. Unset it to download the default Hugging Face model, or point it at a .gguf file.",
        path.display()
    );
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    use rusty_llama::app::*;

    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

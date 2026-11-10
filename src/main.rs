use std::path::PathBuf;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebContext, WebViewBuilder};

// Función auxiliar para cargar el icono de la ventana de forma segura
fn load_icon(path: &str) -> Option<tao::window::Icon> {
    let image = image::open(path).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    tao::window::Icon::from_rgba(rgba, width, height).ok()
}

fn main() -> wry::Result<()> {
    // Parámetros de optimización agresiva de memoria desde el inicio (WebView2)
    std::env::set_var(
        "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
        "--disable-features=Translate,InterestFeedContentSuggestions,MediaRouter,AutofillServerCommunication \
         --disable-background-networking \
         --disable-sync \
         --enable-low-end-device-mode"
    );

    // 1. Configurar la ruta de persistencia en AppData\Local\rumium de Windows
    let mut data_dir = PathBuf::from(
        std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".into())
    );
    data_dir.push("rumium");

    // Crear el contexto web persistente para AppData
    let mut web_context = WebContext::new(Some(data_dir));

    // 2. Inicializar el bucle de eventos del sistema operativo
    let event_loop = EventLoop::new();
    
    // Cargar el icono personalizado (Asegúrate de tener icono.png o icono.ico en la carpeta del proyecto)
    let icon = load_icon("icono.ico").or_else(|| load_icon("icono.png"));

    // 3. Construir la ventana principal de Rumium Browser con el icono asignado
    let mut window_builder = WindowBuilder::new()
        .with_title("Rumium Browser [Open Source] — Ultra Optimizado & AdBlock | [Alt+←/→: Historial]")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 720.0));

    if let Some(ico) = icon {
        window_builder = window_builder.with_window_icon(Some(ico));
    }

    let window = window_builder.build(&event_loop).unwrap();

    // 4. Script de Bloqueo de Anuncios, Historial y Optimizador por inyección JS
    let browser_features_script = r#"
        window.addEventListener('DOMContentLoaded', () => {
            const adSelectors = [
                'iframe[src*="ads"]', 
                'iframe[src*="doubleclick"]', 
                '.adsbygoogle', 
                '[id*="google_ads"]', 
                '.ad-container', 
                'div[class*="banner-ads"]', 
                'div[class*="cookie-banner"]'
            ];
            
            const removeAds = () => {
                adSelectors.forEach(selector => {
                    document.querySelectorAll(selector).forEach(el => el.remove());
                });
            };

            removeAds();
            const observer = new MutationObserver(removeAds);
            observer.observe(document.body, { childList: true, subtree: true });
        });

        window.addEventListener('keydown', (e) => {
            if (e.altKey && e.key === 'ArrowLeft') {
                history.back();
            }
            if (e.altKey && e.key === 'ArrowRight') {
                history.forward();
            }
        });
    "#;

    // 5. Acoplar el motor WebView
    let _webview = WebViewBuilder::new(&window)
        .with_web_context(&mut web_context)
        .with_initialization_script(browser_features_script)
        .with_url("https://www.google.com")
        .build()?;

    // 6. Ejecutar el bucle principal de la aplicación
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
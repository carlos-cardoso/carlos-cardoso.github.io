use dioxus::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    About {},
    #[route("/projects")]
    Projects {},
    #[route("/publications")]
    Publications {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const PHOTO: Asset = asset!("/assets/photo2.jpg");
const FERRIS: Asset = asset!("/assets/Original_Ferris.svg");
// const LINKEDIN: Asset = asset!("/assets/linkedin-svgrepo-com.svg");
// const GITHUB: Asset = asset!("/assets/github-svgrepo-com.svg");
const CV: Asset = asset!("/assets/cv.pdf");

fn main() {
    dioxus::launch(App);
}

// #[server(endpoint = "static_routes")]
// async fn static_routes() -> Result<Vec<String>, ServerFnError> {
//     Ok(Route::static_routes()
//         .into_iter()
//         .map(|route| route.to_string())
//         .collect::<Vec<_>>())
// }

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(SelectedTags(HashSet::new())));
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "og:tyoe", href: TAILWIND_CSS }
        document::Meta { property: "og:title", content: "Carlos Eduardo Cardoso"}
        document::Meta { property: "og:type", content: "website"}
        document::Meta { property: "og:url", content: "https://carlos-cardoso.fly.dev/"}
        document::Meta { property: "og:image", content: "https://carlos-cardoso.fly.dev/assets/preview-image.jpg"}
        document::Meta { property: "og:image:width", content: "1224"}
        document::Meta { property: "og:image:height", content: "792"}
        document::Meta { property: "og:description", content: "Carlos Eduardo Cardoso - Software Engineer"}
        document::Meta { property: "og:locale", content: "en_US"}
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn About() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Header {}
            main { class: "flex-grow container mx-auto px-4 py-8",
                div {
                    img {
                        src: PHOTO,
                        id: "photo",
                        class: "w-64 h-64 rounded-full border shadow-md md:mr-6 mb-4 md:mb-6",
                    }
                }
                h2 { class: "text-xl font-semibold",
                    "I'm currently a Rust Software Engineer accelerating healthcare with AI @ "
                    a {
                        class: "text-blue-500 hover:underline",
                        href: "https://deepc.ai",
                        target: "_blank",
                        "deepc"
                    }
                }
                p { class: "mt-2", "Skills:" }
                ul { class: "list-disc list-inside mt-2 space-y-1",
                    li { "Rust, C/C++, Python" }
                    li { "Kubernetes, Linux/Unix, Git" }
                    li { "PostgreSQL" }
                    li { "Robotics/Machine Learning/Embedded" }
                }
                p { class: "mt-4",
                    "Please see my "
                    Link {
                            class: "text-blue-500 hover:underline",
                            to: Route::Projects {  },
                            "projects"
                    }
                    " and "
                    Link {
                            class: "text-blue-500 hover:underline",
                            to: Route::Publications {  },
                            "publications"
                    }
                    "."
                }
            } //main
            Footer {}
        } //div
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "bg-white py-4 border-b shadow-md",
            div { class: "text-center",
                h1 { class: "text-2xl font-bold", "Carlos Eduardo Cardoso" }
                nav { class: "flex justify-center space-x-4 mt-2",
                    Link {
                            class: "text-blue-500 hover:underline",
                            to: Route::About {  },
                            "About"
                    }
                    Link {
                            class: "text-blue-500 hover:underline",
                            to: Route::Projects {  },
                            "Projects"
                    }
                    Link {
                            class: "text-blue-500 hover:underline",
                            to: Route::Publications {  },
                            "Publications"
                    }
                    a { class: "text-blue-500 hover:underline", href: CV, "CV" }
                }
            }
        }
    }
}

#[component]
pub fn Publications() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen",
            Header {}
            main { class: "flex-grow container font-sans text-gray-800 px-6 py-8 mx-auto max-w-3xl",
                h1 { class: "text-3xl font-bold text-left mb-6", "Publications" }
                ul { class: "space-y-4 list-disc list-inside",
                    // Publication 1
                    li { class: "text-sm",
                        span { "IEEE‑RAS ICRA 2015 Seattle, USA. " }
                        "2015 • C. Cardoso, L. Jamone and A. Bernardino, A novel approach to dynamic movement imitation based on quadratic programming. (first author) "
                        a {
                            class: "text-blue-500 hover:underline",
                            href: "https://ieeexplore.ieee.org/iel7/7128761/7138973/07139285.pdf",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "link"
                        }
                    }

                    // Publication 2
                    li { class: "text-sm",
                        span { "MSc Thesis @ Técnico Lisboa, 2016 " }
                        "• C. Cardoso, \"Robot Skills: Imitation and Exploration Learning - Dynamic Movement Primitives and Reinforcement Learning for a Ping Pong playing Robot\" "
                        a {
                            class: "text-blue-500 hover:underline",
                            href: "https://fenix.tecnico.ulisboa.pt/downloadFile/281870113702919/resumo.pdf",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "link"
                        }
                    }

                    // Publication 3
                    li { class: "text-sm",
                        span { "IEEE‑RAS ICARSC 2017 Coimbra, Portugal. " }
                        "2017 • C. Cardoso and A. Bernardino, Adaptive Non‑Maximal Suppression Filtering for Online Exploration Learning with Cost‑Regularized Kernel Regression. (first author) "
                        a {
                            class: "text-blue-500 hover:underline",
                            href: "https://ieeexplore.ieee.org/iel7/7957705/7964034/07964087.pdf",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "link"
                        }
                    }

                    // Publication 4
                    li { class: "text-sm",
                        span { "Workshop at RO‑MAN 2017 Lisboa, Portugal. " }
                        "• Avelino, Joao, Paulino, Tiago, Cardoso, Carlos, Nunes, Ricardo, Moreno, Plinio and Bernardino, Alexandre. \"Human‑aware natural handshaking using tactile sensors for Vizzy, a social robot.\" (co‑author) "
                        a {
                            class: "text-blue-500 hover:underline",
                            href: "https://vislab.isr.tecnico.ulisboa.pt/wp-content/uploads/2017/11/javelino-romanws2017.pdf",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "link"
                        }
                    }

                    // Publication 5
                    li { class: "text-sm",
                        span { "Paladyn Journal of Behavioral Robotics 2018 " }
                        "• Avelino, João, Paulino, Tiago, Cardoso, Carlos, Nunes, Ricardo, Moreno, Plinio and Bernardino, Alexandre. \"Towards natural handshakes for social robots: human‑aware hand grasps using tactile sensors.\" (co‑author) "
                        a {
                            class: "text-blue-500 hover:underline",
                            href: "https://www.degruyter.com/document/doi/10.1515/pjbr-2018-0017/pdf",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "link"
                        }
                    }

                    // Publication 6
                    li { class: "text-sm",
                        span { "IEEE ICDL‑EpiRob 2019 Oslo, Norway. " }
                        "• A. Dehban*, C. Cardoso*, P. Vicente, A. Bernardino and J. Santos‑Victor, \"Robotic Interactive Physics Parameters Estimator (RIPPE),\" (co‑first author, *equal contribution) "
                        a {
                            class: "text-blue-500 hover:underline",
                            href: "https://vislab.isr.tecnico.ulisboa.pt/wp-content/uploads/2019/06/adehban-icdl2019.pdf",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "link"
                        }
                    }

                    // Publication 7
                    li { class: "text-sm",
                        span { "EgoVIP workshop at IROS 2021 Prague, Czech Republic. " }
                        "• C. Cardoso*, A. Bernardino, \"Bayesian Interaction Primitives for Robot to Human Handover with Giver‑Egocentric Observations\" (first author)"
                    }

                    // Publication 8
                    li { class: "text-sm",
                        span { "IEEE ICDL 2022 London, United Kingdom. " }
                        "• C. Cardoso*, A. Bernardino, \"Exploiting a Statistical Body Model for Handover Interaction Primitives\" (first author)"
                    }
                } //ul
            } //main
            Footer {}
        }
    }
}

#[derive(PartialEq, Clone)]
struct Project {
    image_path: Asset,
    title: &'static str,
    description: &'static str,
    link: &'static str,
    tags: Vec<&'static str>,
}

#[derive(Clone, Debug)]
struct SelectedTags(HashSet<String>);

#[component]
pub fn Projects() -> Element {
    // Define the projects
    let projects = [
    Project {
        image_path: asset!("/assets/flim.png"),
        title: "Overlap Of FLIM Microscopy Images",
        description: "A web app (also native for linux, windows) that computes the pixel overlap of multiple FLIM (Fluorescence Lifetime Imaging Microscopy) images. Allows selecting an area to compute the overlap, replacing colors, and setting the weight of individual images. Can save the blended images and a mask of the overlapping pixels.",
        link: "https://flim-measure-overlap.fly.dev/",
        tags: vec!["Rust", "WASM", "Image Processing", "egui"],
    },
    Project {
        image_path: asset!("/assets/vizzy_hintt.jpg"),
        title: "Vizzy Robot Handshake",
        description: "Created RVIZ plugins and operated the Vizzy Robot for handshaking and handover of the envelope with the GLINTT HINTT health award recipients. Operated Vizzy while handshaking dignitaries such as the President of the European Parliament Antonio Tajani.",
        link: "https://github.com/vislab-tecnico-lisboa/vizzy",
        tags: vec!["C++", "ROS", "RVIZ"],
    },
    Project {
        image_path: asset!("/assets/smpl_space.gif"),
        title: "SMPL domain Interactive Visualizer",
        description: "A simple web app to interactively explore the degrees of freedom of the SMPL model. Based on the visualization code from EasyMocap.",
        link: "https://github.com/carlos-cardoso/SmplDofApp",
        tags: vec!["Python", "HTML"],
    },
    Project {
        image_path: asset!("/assets/vizzy_docker.jpg"),
        title: "Vizzy Robot Simulator Docker Image",
        description: "This docker image allows researchers to skip setting up Vizzy and start experimenting on the simulated Vizzy in a fraction of the time.",
        link: "https://github.com/carlos-cardoso/vizzy-docker",
        tags: vec!["Dockerfiles"],
    },
    Project {
        image_path: asset!("/assets/table_tennis.gif"),
        title: "Learning Robot Table Tennis",
        description: "A robot that learns how to hit a table tennis ball autonomously through exploration from a small set of initial demonstrations.",
        link: "https://github.com/carlos-cardoso/robot-skills",
        tags: vec!["Julia", "C++", "Python", "ROS"],
    },
    Project {
        image_path: asset!("/assets/kinect.jpg"),
        title: "Nix ROS Kinect V2",
        description: "To capture RGBD data in Linux with Kinect V2, this repo contains scripts and the missing nix packages to run libfreenect2 in any Linux system.",
        link: "https://github.com/carlos-cardoso/kinect2-nix-ros-overlay",
        tags: vec!["Nix", "Bash"],
    },
    Project {
        image_path: asset!("/assets/chanters.jpg"),
        title: "3D Printed Galician Bagpipe Chanter",
        description: "Modeled and printed a functional Galician Bagpipe Chanter with the help of Professor Paulo Marinho.",
        link: "https://github.com/carlos-cardoso/galician-chanter-scad",
        tags: vec!["OpenScad"],
    },
    Project {
        image_path: asset!("/assets/teensy_pipe.jpg"),
        title: "Teensy Electronic Bagpipe Chanter",
        description: "An electronic Galician bagpipe chanter based on a Teensy LC board. Has configurable fingering and sends MIDI commands to a synthesizer through USB.",
        link: "https://github.com/carlos-cardoso/teensy-pipe",
        tags: vec!["Embedded", "Python"],
    },
    Project {
        image_path: asset!("/assets/kinova.gif"),
        title: "Physics Parameters Estimator",
        description: "Used a simulation environment (pybullet) to estimate the physical properties of objects (mass, friction) from observations of a robot interacting in the real world.",
        link: "https://github.com/carlos-cardoso/RIPPE",
        tags: vec!["Python"],
    },
    Project {
        image_path: asset!("/assets/tree.jpg"),
        title: "Embedded Behavior Trees",
        description: "An example platformio project for the Arduino due. Uses the beehive header-only behavior trees library and Groot for visual editing of trees.",
        link: "https://github.com/carlos-cardoso/arduino-behavior-tree",
        tags: vec!["Embedded", "Python", "C++"],
    },
    Project {
        image_path: asset!("/assets/1_rg6_nov_capturingyou-cana-liborio.png"),
        title: "Special FX Robot",
        description: "Live-coded a videogame and created a wifi-operated self-destructing robot for the performance 'Capturing you | Fictional Politics of Movement' by Ana Libório.",
        link: "https://ruadasgaivotas6.pt/events/capturing-you-fictional-politics-of-movement/?lang=en",
        tags: vec!["Embedded","Bash", "C++", "Godot"],
    },
    Project {
        image_path: asset!("/assets/passepartout.jpg"),
        title: "I've seen this face before (details)",
        description: "One of two art pieces created in cooperation with artist Bruno José Silva. In display at the Project Room in Banco das Artes Galeria, Leiria.",
        link: "https://brunojosesilva.com/LIMIT-OF-DISAPPEARANCE",
        tags: vec!["Embedded", "HD LCD", "Camera", "OpenCV", "PyGame", "Python"],
    },
    ];

    // State for the selected tags
    let mut selected_tags = consume_context::<Signal<SelectedTags>>();

    // Filtered projects based on selected tags
    let filtered_projects = projects
        .iter()
        .filter(|project| {
            selected_tags.read().0.is_empty()
                || project
                    .tags
                    .iter()
                    .any(|tag| selected_tags.read().0.contains(*tag))
        })
        .cloned()
        .collect::<Vec<_>>();

    let mut all_tags: HashSet<String> = HashSet::new();
    for p in projects {
        for t in p.tags {
            all_tags.insert(t.to_string());
        }
    }
    let mut all_tags: Vec<String> = all_tags.into_iter().collect();
    all_tags.sort();

    rsx! {

        div { class: "flex flex-col min-h-screen",
            Header {}
            main { class: "flex-grow container mx-auto px-4 py-8",
                //div { class: " flex flex-col bg-white min-h-screen font-sans text-gray-800 px-6 py-8 mx-auto max-w-6xl",
                h1 { class: "text-all_tags3xl font-bold text-left mb-6", "Projects" }

                // Tag Filter Section
                div { class: "mb-6",
                    h2 { class: "text-lg font-semibold mb-2", "Filter by Tags:" }
                    div { class: "flex flex-wrap gap-2",
                        for tag in all_tags {
                            button {
                                class: "px-3 py-1 rounded-full text-sm font-medium border text-gray-700 hover:bg-blue-100 transition",
                                style: if selected_tags.read().0.contains(&tag.to_string()) { "background-color: #cce5ff;" } else { "" },
                                onclick: move |_| {
                                    let mut tags = selected_tags.read().0.clone();
                                    if !tags.insert(tag.to_string()) {
                                        tags.remove(&tag.to_string());
                                    }
                                    selected_tags.write().0 = tags;
                                },
                                "{tag}"
                            }
                        }
                    }
                }

                // Projects Display Section
                div { class: "grid grid-cols-1 gap-4",
                    for project in filtered_projects {
                        div { class: "grid grid-cols-5 items-center gap-4 border rounded-lg p-4",
                            div { class: "col-span-4",
                                h3 { class: "text-lg font-semibold mb-2", "{project.title}" }
                                p { class: "text-sm text-gray-600 mb-4", "{project.description}" }
                                a {
                                    href: "{project.link}",
                                    target: "_blank",
                                    class: "text-blue-500 hover:underline mb-2 inline-block",
                                    "Open Project"
                                }

                                // Tags Section
                                div { class: "flex flex-wrap gap-2",
                                    for tag in project.tags.iter() {
                                        span {
                                            class: "px-3 py-1 rounded-full text-sm font-medium border ".to_owned()
                                                + if selected_tags.read().0.contains(*tag) {
                                                    "bg-blue-100 text-blue-800 border-blue-300"
                                                } else {
                                                    "bg-gray-100 text-gray-700 border-gray-300"
                                                },
                                            "{tag}"
                                        }
                                    }
                                }


                            }
                            img {
                                src: "{project.image_path}",
                                alt: "{project.title}",
                                class: "w-64 h-64 object-cover col-span-1 rounded-md",
                            }

                        }
                    }
                }
            }
            Footer {}
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-gray-100 py-4 border-t shadow-inner",
            div { class: "text-center text-sm text-gray-600",
                p { "Carlos Eduardo Cardoso" }
                p { "carlos.cardoso@tecnico.ulisboa.pt" }
                div { class: "flex justify-center space-x-4 mt-2",
                    a {
                        class: "text-blue-500 hover:underline flex items-center space-x-2",
                        href: "https://github.com/carlos-cardoso",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        // GitHub Link with Icon
                        svg {
                            class: "w-5 h-5",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            view_box: "0 0 24 24",
                            path { d: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577v-2.234c-3.338.724-4.033-1.415-4.033-1.415-.546-1.387-1.333-1.757-1.333-1.757-1.087-.744.083-.729.083-.729 1.205.084 1.84 1.236 1.84 1.236 1.07 1.835 2.809 1.305 3.495.997.108-.775.418-1.305.762-1.605-2.665-.304-5.466-1.332-5.466-5.931 0-1.31.47-2.38 1.236-3.22-.124-.303-.536-1.523.116-3.176 0 0 1.008-.322 3.3 1.23.957-.266 1.983-.399 3.005-.405 1.022.006 2.048.139 3.007.405 2.29-1.552 3.297-1.23 3.297-1.23.653 1.653.241 2.873.118 3.176.77.84 1.235 1.91 1.235 3.22 0 4.61-2.807 5.625-5.479 5.922.43.372.823 1.103.823 2.222v3.293c0 .319.216.694.824.576 4.765-1.584 8.2-6.081 8.2-11.382 0-6.627-5.373-12-12-12z" }
                        }
                        span { "GitHub" }
                    } //a
                    a {
                        class: "text-blue-500 hover:underline flex items-center space-x-2",
                        href: "https://linkedin.com/in/cesfcardoso",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        // LinkedIn Link with Icon
                        svg {
                            class: "w-5 h-5",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            view_box: "0 0 24 24",
                            path { d: "M22.225 0H1.771C.792 0 0 .774 0 1.73v20.541C0 23.226.792 24 1.771 24h20.451c.978 0 1.778-.774 1.778-1.729V1.73C24 .774 23.203 0 22.225 0zM7.124 20.452H3.556V9h3.568v11.452zm-1.784-13c-1.13 0-2.043-.926-2.043-2.059 0-1.134.914-2.059 2.043-2.059s2.043.926 2.043 2.059c0 1.134-.914 2.059-2.043 2.059zM20.452 20.452h-3.568V14.84c0-1.342-.025-3.067-1.868-3.067-1.868 0-2.156 1.459-2.156 2.966v5.713h-3.568V9h3.423v1.561h.049c.477-.897 1.637-1.841 3.372-1.841 3.605 0 4.27 2.372 4.27 5.458v6.274z" }
                        }
                        span { "LinkedIn" }
                    } //a
                } //div
                div { class: "flex justify-center items-center space-x-2 mt-4",
                    img {
                        class: "w-6 h-6", // Adjust size of Ferris icon
                        src: FERRIS, // Path to Ferris SVG
                        alt: "Ferris, the Rust mascot",
                    }
                    span { class: "text-gray-600", "Built with Rust and Dioxus" }
                    img { class: "w-6 h-6", src: FAVICON, alt: "Dioxus Rust" }
                } //div
            } //div
        } //footer
    } //rsx
}

// /// Echo component that demonstrates fullstack server functions.
// #[component]
// fn Echo() -> Element {
//     let mut response = use_signal(|| String::new());

//     rsx! {
//         div { id: "echo",
//             h4 { "ServerFn Echo" }
//             input {
//                 placeholder: "Type here to echo...",
//                 oninput: move |event| async move {
//                     let data = echo_server(event.value()).await.unwrap();
//                     response.set(data);
//                 },
//             }

//             if !response().is_empty() {
//                 p {
//                     "Server echoed: "
//                     i { "{response}" }
//                 }
//             }
//         }
//     }
// }

// /// Echo the user input on the server.
// #[server(EchoServer)]
// async fn echo_server(input: String) -> Result<String, ServerFnError> {
//     Ok(input)
// }

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

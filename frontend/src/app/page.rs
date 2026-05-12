use leptos::{logging, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub floor: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampusNode {
    #[serde(rename = "nodeId")]
    pub node_id: String,

    pub name: String,

    #[serde(rename = "nodeType")]
    pub node_type: String,

    pub coordinates: Coordinates,
}

#[component]
pub fn App() -> impl IntoView {
    let (start, set_start) = signal("BU_MAIN_GATE".to_string());

    let (end, set_end) = signal("A_GF_DIRECTOR".to_string());

    let path = LocalResource::new(move || {
        let s = start.get();
        let e = end.get();

        async move {
            let url = format!(
                "http://127.0.0.1:3000/path?start={}&end={}",
                s,
                e
            );

            reqwest::get(url)
    .await
    .ok()?
    .json::<Vec<CampusNode>>()
    .await
    .ok()
    .inspect(|nodes| {
        logging::log!("{:?}", nodes);
    })
        }
    });

    let route_path = move || {
        path.get()
            .and_then(|nodes| {
                nodes.map(|nodes| {
                    let mut d = String::new();

                    for (i, node) in nodes.iter().enumerate() {
                        let x = node.coordinates.x * 4.5 + 1200.0;
                        let y = node.coordinates.y * 4.0 + 500.0;

                        if i == 0 {
                            d.push_str(&format!("M {} {}", x, y));
                        } else {
                            d.push_str(&format!(" L {} {}", x, y));
                        }
                    }

                    d
                })
            })
            .unwrap_or_default()
    };


    let locations = vec![
    ("BU_MAIN_GATE", "Main Gate"),
    ("A_GF_ENTRANCE", "Block A Entrance"),
    ("A_GF_DIRECTOR", "Director Office"),
    ("A_GF_CS_HOD", "CS HOD"),
    ("A_GF_STAFF_ROOM", "Staff Room"),
    ("B_GF_CS_LAB_104", "CS Lab 104"),
    ("B_GF_MECH_LAB", "Mechanical Lab"),
    ("B_GF_MECH_WORKSHOP", "Workshop"),
    ("B_GF_DRAWING_HALL", "Drawing Hall"),
];

    view! {
        <div class="app-shell">

            <header class="topbar">
                <h1>"BU Navigation System"</h1>
            </header>

            <div class="content">

                <aside class="sidebar">

                    <h2>"Navigation"</h2>

                    <div class="input-group">
                        <label>"Start"</label>

                        <select
    on:change=move |ev| {
        set_start.set(event_target_value(&ev));
    }
>
    {
        locations.iter().map(|(id, name)| {
            view! {
                <option value=*id>
                    {name.to_string()}
                </option>
            }
        }).collect_view()
    }
</select>
                    </div>

                    <div class="input-group">
                        <label>"Destination"</label>

                        <select
    
>
    {
        locations.iter().map(|(id, name)| {
            view! {
                <option value=*id>
                    {name.to_string()}
                </option>
            }
        }).collect_view()
    }
</select>
                    </div>

                </aside>

                <main class="map-area">

                    <div class="map-wrapper">

                        <img
                            src="/assets/maps/b.svg"
                            class="map-svg"
                        />

                        <svg
                            class="route-overlay"
                            viewBox="0 0 4688 4674"
                        >

                            <path
                                d=route_path
                                class="active-route"
                            />

                        </svg>

                    </div>

                </main>

            </div>

        </div>
    }
}
use leptos::{logging, prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

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

#[derive(Clone)]
pub struct LocationOption {
    pub id: &'static str,
    pub name: &'static str,
}

#[component]
pub fn App() -> impl IntoView {
    let (start, set_start) = signal("BU_MAIN_GATE".to_string());

    let (end, set_end) = signal("A_GF_DIRECTOR".to_string());

    let (current_map, set_current_map) =
    signal("/assets/maps/outer.svg".to_string());

    let (route_data, set_route_data) =
    signal(String::new());

    let mut routes_map: HashMap<
    (&str, &str),
    &str
> = HashMap::new();
let visual_routes = Rc::new(routes_map);
   let navigate = move |_| {

    let visual_routes = visual_routes.clone();

    let s = start.get_untracked();
    let e = end.get_untracked();

    spawn_local(async move {

        let url = format!(
            "http://127.0.0.1:3000/path?start={}&end={}",
            s,
            e
        );

        if let Ok(resp) = reqwest::get(&url).await {

            if let Ok(nodes) =
                resp.json::<Vec<CampusNode>>().await {

                let mut final_path = String::new();

                for pair in nodes.windows(2) {

                    let from = &pair[0].node_id;
                    let to = &pair[1].node_id;

                    if let Some(segment) =
                        visual_routes.get(
                            &(from.as_str(), to.as_str())
                        )
                    {

                        final_path.push_str(segment);
                        final_path.push(' ');

                    } else if let Some(segment) =
                        visual_routes.get(
                            &(to.as_str(), from.as_str())
                        )
                    {

                        final_path.push_str(segment);
                        final_path.push(' ');
                    }
                }

                set_route_data.set(final_path);

                if e.starts_with("B_") {

                    set_current_map.set(
                        "/assets/maps/b.svg".to_string()
                    );

                } else if e.starts_with("A_1F_") {

                    set_current_map.set(
                        "/assets/maps/block_a_first.svg"
                            .to_string()
                    );

                } else if e.starts_with("A_") {

                    set_current_map.set(
                        "/assets/maps/block_a_ground.svg"
                            .to_string()
                    );

                } else {

                    set_current_map.set(
                        "/assets/maps/outer.svg"
                            .to_string()
                    );
                }
            }
        }
    });
};


    let locations: Vec<LocationOption> = vec![

    LocationOption {
        id: "BU_MAIN_GATE",
        name: "BU Main Gate",
    },

    LocationOption {
        id: "UIT_ENGG_CAMPUS_ENTRY",
        name: "UIT Engineering Campus Entry",
    },

    LocationOption {
        id: "A_GF_ENTRANCE",
        name: "Block A Entrance",
    },

    LocationOption {
        id: "A_GF_DIRECTOR",
        name: "Director Office",
    },

    LocationOption {
        id: "A_GF_CS_HOD",
        name: "CS HOD Office",
    },

    LocationOption {
        id: "A_GF_IT_HOD",
        name: "IT HOD Office",
    },

    LocationOption {
        id: "A_GF_EC_HOD",
        name: "EC HOD Office",
    },

    LocationOption {
        id: "A_GF_ME_HOD",
        name: "Mechanical HOD Office",
    },

    LocationOption {
        id: "A_GF_CIVIL_HOD",
        name: "Civil HOD Office",
    },

    LocationOption {
        id: "A_GF_STAFF_ROOM",
        name: "Staff Room",
    },

    LocationOption {
        id: "A_GF_ADV_IT_LAB",
        name: "Advanced IT Lab",
    },

    LocationOption {
        id: "A_GF_SERVER_ROOM",
        name: "Server Room",
    },

    LocationOption {
        id: "A_GF_SEMINAR_HALL",
        name: "Seminar Hall",
    },

    LocationOption {
        id: "A_GF_OFFICE",
        name: "Administrative Office",
    },

    LocationOption {
        id: "A_1F_LIBRARY",
        name: "Library",
    },

    LocationOption {
        id: "A_1F_READING_ROOM",
        name: "Reading Room",
    },

    LocationOption {
        id: "A_1F_SMART_CLASS",
        name: "Smart Classroom",
    },

    LocationOption {
        id: "A_1F_FACULTY_ROOM",
        name: "Faculty Room",
    },

    LocationOption {
        id: "A_1F_PROJECT_LAB",
        name: "Project Lab",
    },

    LocationOption {
        id: "B_GF_ENTRANCE",
        name: "Block B Entrance",
    },

    LocationOption {
        id: "B_GF_CS_LAB_101",
        name: "CS Lab 101",
    },

    LocationOption {
        id: "B_GF_CS_LAB_102",
        name: "CS Lab 102",
    },

    LocationOption {
        id: "B_GF_CS_LAB_103",
        name: "CS Lab 103",
    },

    LocationOption {
        id: "B_GF_CS_LAB_104",
        name: "CS Lab 104",
    },

    LocationOption {
        id: "B_GF_MECH_LAB",
        name: "Mechanical Lab",
    },

    LocationOption {
        id: "B_GF_MECH_WORKSHOP",
        name: "Mechanical Workshop",
    },

    LocationOption {
        id: "B_GF_DRAWING_HALL",
        name: "Engineering Drawing Hall",
    },

    LocationOption {
        id: "B_GF_MATH_HALL",
        name: "Math Hall",
    },

    LocationOption {
        id: "B_GF_MATH_STAFF",
        name: "Math Staff Room",
    },

    LocationOption {
        id: "B_GF_PHYSICS_LAB",
        name: "Physics Lab",
    },

    LocationOption {
        id: "B_GF_CHEMISTRY_LAB",
        name: "Chemistry Lab",
    },

    LocationOption {
        id: "B_GF_SEMINAR_ROOM",
        name: "Seminar Room",
    },

    LocationOption {
        id: "B_GF_WORKSHOP_STORE",
        name: "Workshop Store",
    },

    LocationOption {
        id: "CENTRAL_LIBRARY",
        name: "Central Library",
    },

    LocationOption {
        id: "ADMIN_BLOCK",
        name: "Administrative Block",
    },

    LocationOption {
        id: "AUDITORIUM",
        name: "Auditorium",
    },

    LocationOption {
        id: "SPORTS_COMPLEX",
        name: "Sports Complex",
    },

    LocationOption {
        id: "CAFETERIA",
        name: "Cafeteria",
    },

    LocationOption {
        id: "PARKING_AREA",
        name: "Parking Area",
    },

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
                        prop:value=start
                        on:change=move |ev| {
                            set_start.set(
                                event_target_value(&ev)
                            );
                        }
                    >

                        {
                            locations.iter().map(|loc| {

                                view! {
                                    <option value=loc.id>
                                        {loc.name}
                                    </option>
                                }

                            }).collect_view()
                        }

                    </select>

                </div>

                <div class="input-group">

                    <label>"Destination"</label>

                    <select
                        prop:value=end
                        on:change=move |ev| {
                            set_end.set(
                                event_target_value(&ev)
                            );
                        }
                    >

                        {
                            locations.iter().map(|loc| {

                                view! {
                                    <option value=loc.id>
                                        {loc.name}
                                    </option>
                                }

                            }).collect_view()
                        }

                    </select>

                </div>

                <button
                    class="navigate-btn"
                    on:click=navigate
                    type="button"
                >
                    "Navigate"
                </button>

                <div class="map-buttons">

                    <button
                        type="button"
                        on:click=move |_| {

                            set_current_map.set(
                                "/assets/maps/outer.svg"
                                    .to_string()
                            )

                        }
                    >
                        "Campus"
                    </button>

                    <button
                        type="button"
                        on:click=move |_| {

                            set_current_map.set(
                                "/assets/maps/block_a_ground.svg"
                                    .to_string()
                            )

                        }
                    >
                        "Block A GF"
                    </button>

                    <button
                        type="button"
                        on:click=move |_| {

                            set_current_map.set(
                                "/assets/maps/block_a_first.svg"
                                    .to_string()
                            )

                        }
                    >
                        "Block A 1F"
                    </button>

                    <button
                        type="button"
                        on:click=move |_| {

                            set_current_map.set(
                                "/assets/maps/b.svg"
                                    .to_string()
                            )

                        }
                    >
                        "Block B"
                    </button>

                </div>

            </aside>

            <main class="map-area">

                <div class="map-wrapper">

                    <img
                        src=move || current_map.get()
                        class="map-svg"
                    />

                    <svg
                        class="route-overlay"
                        viewBox="0 0 4688 4674"
                    >

                        <path
                            d=move || route_data.get()
                            class="active-route"
                        />

                    </svg>

                </div>

            </main>

        </div>

    </div>
}
}
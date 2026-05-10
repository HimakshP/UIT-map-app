use leptos::*;
use serde::{Deserialize, Serialize};

// Ensure this matches the struct in your backend/models.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampusNode {
    pub node_id: String,
    pub name: String,
    pub node_type: String,
    pub coordinates: Coordinates,
    pub floor: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
}

#[component]
pub fn App() -> impl IntoView {
    // 1. Reactive state for navigation inputs
    let (start_id, set_start_id) = create_signal("BU_MAIN_GATE".to_string());
    let (end_id, set_end_id) = create_signal("A_GF_DIRECTOR".to_string());
    let (current_floor, set_current_floor) = create_signal(1);

    // 2. Resource to fetch path data from your Axum backend
    let path_resource = create_resource(
        move || (start_id.get(), end_id.get()),
        |(s, e)| async move {
            let url = format!("http://127.0.0.1:3000/path?start={}&end={}", s, e);
            reqwest::get(url).await.ok()?.json::<Vec<CampusNode>>().await.ok()
        },
    );

    // 3. Derived calculation for total distance
    let total_distance = move || {
        path_resource.get().and_then(|path| {
            path.map(|nodes| {
                // Simplified distance display for the UI
                nodes.len() as f32 * 10.5 
            })
        }).unwrap_or(0.0)
    };

    view! {
        <div class="flex flex-col h-screen bg-[#181818] text-[#e0e0e0] font-sans relative">
            // HEADER
            <header class="flex items-center justify-between bg-[#0088cc] text-white px-4 py-2 text-sm border-b border-[#333333]">
                <div class="flex items-center gap-2">
                    <span class="font-semibold">"BU-Nav - Navigation System"</span>
                </div>
            </header>

            <div class="flex flex-1 overflow-hidden relative">
                // MAIN MAP AREA
                <div class="flex-1 bg-[#1c1c1c] relative overflow-hidden flex items-center justify-center p-8">
                    // Grid Background Overlay
                    <div class="absolute inset-0 pointer-events-none opacity-10"
                        style="background-image: linear-gradient(#444 1px, transparent 1px), linear-gradient(90deg, #444 1px, transparent 1px); background-size: 40px 40px;">
                    </div>

                    // SVG Map Viewer
                    <svg viewBox="0 0 1000 800" class="w-full h-full max-w-4xl max-h-full" style="filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.5))">
                        // Path Polyline (Reactive)
                        {move || path_resource.get().map(|path| {
                            match path {
                                Some(nodes) => {
                                    let points = nodes.iter()
                                        .map(|n| format!("{},{}", n.coordinates.x + 500.0, n.coordinates.y + 400.0))
                                        .collect::<Vec<_>>()
                                        .join(" ");
                                    view! { <polyline points=points fill="none" stroke="#0088cc" stroke-width="4" class="transition-all duration-500" /> }.into_view()
                                }
                                None => view! { }.into_view()
                            }
                        })}
                    </svg>

                    // FLOOR SWITCHER (Bottom Right)
                    <div class="absolute bottom-6 right-6 bg-[#2a2a2a] border border-[#444] rounded-sm flex flex-col overflow-hidden text-[#aaa]">
                        {move || (1..=3).rev().map(|f| {
                            let is_active = move || current_floor.get() == f;
                            view! {
                                <button 
                                    on:click=move |_| set_current_floor.set(f)
                                    class=move || if is_active() { "p-2 bg-[#0088cc] text-white text-xs font-mono" } else { "p-2 hover:bg-[#3a3a3a] border-b border-[#444] text-xs font-mono" }
                                >
                                    {f}
                                </button>
                            }
                        }).collect_view()}
                        <button class="p-2 hover:bg-[#3a3a3a] border-b border-[#444] text-xs font-mono">"G"</button>
                    </div>
                </div>

                // SIDEBAR
                <div class="absolute top-4 left-4 h-[calc(100%-2rem)] shadow-2xl rounded-lg overflow-hidden border border-[#333333]">
                    <div class="w-64 bg-[#222222] flex flex-col text-sm overflow-y-auto h-full opacity-95 backdrop-blur-md">
                        <div class="flex flex-col">
                            <div class="px-3 py-2 text-[#888888] hover:bg-[#333333] cursor-pointer">"Campus Overview"</div>
                            <div class="px-3 py-2 bg-[#0088cc] text-white">"Engineering Bldg (Internal)"</div>
                        </div>

                        <div class="px-3 py-4 border-t border-[#333333] mt-2">
                            <div class="text-xs text-[#888888] mb-3">"Navigation Parameters"</div>
                            <div class="mb-4">
                                <div class="text-xs text-[#888888] mb-1">"Origin Point:"</div>
                                <input 
                                    type="text"
                                    prop:value=start_id
                                    on:input=move |ev| set_start_id.set(event_value(&ev))
                                    class="w-full border border-[#333333] bg-[#252525] rounded-sm px-2 py-1 text-xs text-white" 
                                />
                            </div>
                            <div class="mb-4">
                                <div class="text-xs text-[#888888] mb-1">"Destination:"</div>
                                <input 
                                    type="text"
                                    prop:value=end_id
                                    on:input=move |ev| set_end_id.set(event_value(&ev))
                                    class="w-full border border-[#333333] bg-[#252525] rounded-sm px-2 py-1 text-xs text-white" 
                                />
                            </div>
                        </div>

                        <div class="px-3 py-4 border-t border-[#333333]">
                            <div class="text-xs text-[#888888] mb-3">"Journey Details"</div>
                            <div class="flex flex-col gap-1 text-xs">
                                <div class="flex justify-between">
                                    <span class="text-[#888888]">"Distance:"</span>
                                    <span>{move || format!("{:.1} meters", total_distance())}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-[#888888]">"End Node:"</span>
                                    <span>{move || end_id.get()}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
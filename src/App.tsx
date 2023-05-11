import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
	useEffect(() => {
		let unlisten: any;
		async function get_init_field() {
			unlisten = await listen("InitialField", event => {
				console.log("InitialField", event.payload)
			});
		}

		get_init_field()

		return () => {
			if(unlisten) { unlisten(); }
		}
	}, [])

    return (
        <div className="container">
            <h1>yey</h1>
        </div>
    );
}

export default App;

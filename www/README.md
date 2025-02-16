# Webpage tester for library
Basic Svelte/Vite app utilizing MapLibre to display a basic map with OpenStreetMap tiles. Then there's a big button to upload a KML, displaying whatever polygons are extracted on the map below.

## Running it
Make sure to do a good ol `npm install` (or whatever package manager for js), and then run the `dev` script from the `package.json` (probably `npm run dev`)
Should be exposed locally on 5173 if everything's looking right.

## Vite setup
Worth noting that you gotta include some extra plugins for supporting WASM modules, see the `vite.config.js` to see the setup.
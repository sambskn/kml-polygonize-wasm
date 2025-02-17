<script>
  import { polygonizeKML } from "kml-polygonize-wasm";
  import { Map as MapLibre } from "maplibre-gl";
  import { onDestroy, onMount } from "svelte";
  import { readFileFromUser } from "./fileUpload.js";
  import "maplibre-gl/dist/maplibre-gl.css";

  let polygons = $state([]);
  let map = $state();
  let mapContainerRef = $state();

  const tileUrl = "https://tile.openstreetmap.org/{z}/{x}/{y}.png";

  const callWasm = async () => {
    const { data } = await readFileFromUser(".kml");
    console.log("about to call wasm");
    const res = polygonizeKML({ kml: data });
    console.log("new polygons", res.polygons);
    polygons = res.polygons;
  };

  $effect(() => {
    if (!map) return;
    // remove any existing polygon layers from map
    const currentMapIds = map.getLayersOrder();
    for (const layerId of currentMapIds) {
      if (layerId.includes("polygon-")) {
        map.removeLayer(layerId);
        map.removeSource(layerId);
      }
    }
    // add whatever polygons we got
    let i = 0;
    for (const polygon of polygons) {
      const baseId = `polygon-${i}`;
      map.addSource(baseId, {
        type: "geojson",
        data: {
          type: "Feature",
          geometry: {
            type: "Polygon",
            coordinates: polygon.points,
          },
        },
      });
      map.addLayer({
        id: baseId,
        type: "fill",
        source: baseId,
        layout: {},
        paint: {
          "fill-color": "#088",
          "fill-opacity": 0.8,
        },
      });
      i++;
    }
    if (polygons.length > 0) {
      map.flyTo({
        center: polygons[0].points[0][0],
      });
    }
  });

  onMount(() => {
    const initialState = { lng: -92.44634703, lat: 32.593912998, zoom: 14 };
    map = new MapLibre({
      container: mapContainerRef,
      style: {
        version: 8,
        name: "MapLibre Demo Tiles",
        sprite:
          "https://demotiles.maplibre.org/styles/osm-bright-gl-style/sprite",
        glyphs: "https://demotiles.maplibre.org/font/{fontstack}/{range}.pbf",
        sources: {},
        layers: [],
      },
      center: [initialState.lng, initialState.lat],
      zoom: initialState.zoom,
    });
    map.on("load", () => {
      map.addSource("openstreetmap", {
        type: "raster",
        tiles: [tileUrl],
        tileSize: 256,
      });
      map.addLayer({
        id: "openstreetmap-layer",
        type: "raster",
        source: "openstreetmap",
      });
    });
  });
  onDestroy(() => {
    map.remove();
  });
</script>

<main>
  <h1>WASM KML Parser Testing</h1>

  <div class="card">
    <button onclick={callWasm}>Click to upload KML and test!</button>
  </div>
  <div class="map-wrap">
    <div class="map-container" bind:this={mapContainerRef}></div>
  </div>
</main>

<style>
  .map-wrap {
    width: 75vw;
    height: 400px;
    overflow: hidden;
    border: solid 2px rgba(50 50 50 / 0.25);
  }
  .map-container {
    height: 100%;
    width: 100%;
  }
</style>

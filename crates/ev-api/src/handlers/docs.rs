use axum::response::Html;

pub async fn api_docs() -> Html<String> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>EV Knowledge Graph API - Documentation</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px 20px; text-align: center; border-radius: 10px; margin-bottom: 30px; }
        h1 { font-size: 2.5em; margin-bottom: 10px; }
        .subtitle { font-size: 1.2em; opacity: 0.9; }
        .section { background: white; padding: 30px; margin-bottom: 20px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h2 { color: #667eea; margin-bottom: 20px; border-bottom: 2px solid #667eea; padding-bottom: 10px; }
        h3 { color: #764ba2; margin: 20px 0 10px; }
        .endpoint { background: #f8f9fa; padding: 15px; margin: 10px 0; border-left: 4px solid #667eea; border-radius: 5px; }
        .method { display: inline-block; padding: 5px 10px; border-radius: 5px; font-weight: bold; margin-right: 10px; }
        .get { background: #28a745; color: white; }
        .post { background: #007bff; color: white; }
        code { background: #f4f4f4; padding: 2px 6px; border-radius: 3px; font-family: 'Courier New', monospace; }
        pre { background: #282c34; color: #abb2bf; padding: 15px; border-radius: 5px; overflow-x: auto; }
        .badge { display: inline-block; padding: 3px 8px; border-radius: 3px; font-size: 0.85em; font-weight: bold; margin-left: 10px; }
        .cache { background: #ffc107; color: #000; }
        .graph { background: #17a2b8; color: white; }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🔋 EV Knowledge Graph API</h1>
            <p class="subtitle">India's Unified Electric Vehicle Intelligence Layer</p>
        </header>

        <div class="section">
            <h2>📖 Overview</h2>
            <p>The EV Knowledge Graph API provides comprehensive data about India's electric vehicle ecosystem including vehicles, chargers, batteries, manufacturers, policies, and compatibility information.</p>
        </div>

        <div class="section">
            <h2>🏥 Health & Status</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/health</code>
                <p>Check API health and service status</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/stats</code>
                <p>Get graph database statistics</p>
            </div>
        </div>

        <div class="section">
            <h2>🚗 Vehicles</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/vehicles</code>
                <span class="badge cache">CACHED</span>
                <p>List all electric vehicles</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/vehicles/:id</code>
                <span class="badge cache">CACHED</span>
                <p>Get vehicle details by ID</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/vehicles/search?q={query}</code>
                <span class="badge cache">CACHED</span>
                <p>Full-text search for vehicles</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/vehicles/oem/:oem_id</code>
                <span class="badge cache">CACHED</span>
                <p>Get all vehicles from a specific manufacturer</p>
            </div>
        </div>

        <div class="section">
            <h2>⚡ Chargers</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/chargers</code>
                <p>List all charging stations</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/chargers/:id</code>
                <span class="badge cache">CACHED</span>
                <p>Get charger details by ID</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/chargers/city/:state/:city</code>
                <span class="badge cache">CACHED</span>
                <p>Get chargers in a specific city</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/chargers/nearby?lat={lat}&lon={lon}&radius={km}</code>
                <span class="badge cache">CACHED</span>
                <p>Find chargers near a location</p>
            </div>
        </div>

        <div class="section">
            <h2>🔌 Compatibility</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/compatibility/vehicle/:vehicle_id</code>
                <span class="badge cache">CACHED</span>
                <span class="badge graph">GRAPH QUERY</span>
                <p>Find all chargers compatible with a vehicle</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/compatibility/charger/:charger_id</code>
                <span class="badge cache">CACHED</span>
                <span class="badge graph">GRAPH QUERY</span>
                <p>Find all vehicles compatible with a charger</p>
            </div>
            <div class="endpoint">
                <span class="method post">POST</span>
                <code>/api/v1/compatibility/check</code>
                <span class="badge graph">GRAPH QUERY</span>
                <p>Check if a vehicle and charger are compatible</p>
                <pre>{ "vehicle_id": "uuid", "charger_id": "uuid" }</pre>
            </div>
        </div>

        <div class="section">
            <h2>📊 Graph Queries</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/query/fame-eligible/:state</code>
                <span class="badge cache">CACHED</span>
                <span class="badge graph">GRAPH QUERY</span>
                <p>Get FAME-II eligible vehicles in a state</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/query/indian-oems</code>
                <span class="badge cache">CACHED</span>
                <span class="badge graph">GRAPH QUERY</span>
                <p>Get all vehicles from Indian manufacturers</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/query/network-coverage/:state</code>
                <span class="badge cache">CACHED</span>
                <span class="badge graph">GRAPH QUERY</span>
                <p>Get charging network coverage by state</p>
            </div>
        </div>

        <div class="section">
            <h2>🧠 Recommendations & Inference</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/recommendations/vehicles?type={type}&min_range={km}&max_price={inr}</code>
                <span class="badge cache">CACHED</span>
                <span class="badge graph">GRAPH QUERY</span>
                <p>Get vehicle recommendations based on criteria</p>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <code>/api/v1/recommendations/chargers?lat={lat}&lon={lon}&vehicle_id={id}</code>
                <span class="badge graph">GRAPH QUERY</span>
                <p>Get recommended chargers for a vehicle near a location</p>
            </div>
        </div>

        <div class="section">
            <h2>💡 Usage Examples</h2>
            <h3>Find compatible chargers for Tata Nexon EV Max</h3>
            <pre>curl http://localhost:8080/api/v1/compatibility/vehicle/{vehicle-uuid}</pre>

            <h3>Search for vehicles</h3>
            <pre>curl http://localhost:8080/api/v1/vehicles/search?q=nexon</pre>

            <h3>Find chargers in Mumbai</h3>
            <pre>curl http://localhost:8080/api/v1/chargers/city/Maharashtra/Mumbai</pre>

            <h3>Get FAME-II eligible vehicles in Delhi</h3>
            <pre>curl http://localhost:8080/api/v1/query/fame-eligible/Delhi</pre>
        </div>

        <div class="section">
            <h2>📝 Response Format</h2>
            <p>All responses follow this format:</p>
            <pre>{
  "success": true,
  "data": { ... },
  "error": null
}</pre>
            <h3>Response Headers</h3>
            <p>All responses include Folonite branding headers:</p>
            <pre>X-Powered-By: Folonite
X-Folonite-Version: 2026-01
X-Folonite-Request-ID: &lt;unique-uuid&gt;</pre>
        </div>
    </div>
</body>
</html>
    "#.to_string())
}

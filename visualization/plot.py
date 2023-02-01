import plotly.express as px 
from APIResponse import JSONData
import geopandas as gpd
import dotenv 


if __name__ == "__main__":
    parsed = dotenv.dotenv_values()
    px.set_mapbox_access_token(parsed['TOKEN'])

    response : JSONData | None = JSONData.get_stations("http://127.0.0.1:8000/stations/get_geodata/{}", 1000)
    if response:
        print(parsed)
        fig = px.scatter_mapbox(data_frame= response.data, lat = response.data.geometry.y , lon = response.data.geometry.x, hover_data=['StationsNaam'], mapbox_style = 'streets')
        fig.update_geos(fitbounds="locations")
        fig.write_html("map.html")

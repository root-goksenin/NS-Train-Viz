import requests 
from typing import List,Any
from dataclasses import dataclass
from geojson import Point
import geopandas as gpd

@dataclass(frozen=True)
class JSONData:
    type : str
    data : gpd.GeoDataFrame
    @classmethod
    def get_stations(cls, url: str, total : int):
        response = requests.get(url = url.format(total) )
        if response:
            parsed = response.json()
            print(parsed.keys())
            return cls(parsed['type'],data = gpd.GeoDataFrame.from_features(parsed['features']))
        else:
            return None

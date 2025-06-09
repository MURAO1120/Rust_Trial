import requests

def get_vehicle_speed():
    try:
        response = requests.get("http://localhost:8080/vehicle/speed")
        response.raise_for_status()
        data = response.json()
        print(f"🚗 現在の車速: {data['speed_kph']:.2f} km/h")
    except requests.RequestException as e:
        print("❌ APIリクエストに失敗しました:", e)

if __name__ == "__main__":
    get_vehicle_speed()

# OpenWeatherCLI
- A command line interface for the openweathermap.org API
- Get weather data for locations around the world from the command line
- Useful for scripting and can be used to fetch data for status bars, dashboards, and more

# Setup (Linux)
- For convenience, included in the root directory is a setup script called setup.sh
- Running this script is not entirely necessary, but it is here to automate the setup process for those who wish to do so
- This script will build the binary and will create a symbolic link to ~/bin/owcli
- This assumes that ~/bin is in your PATH, so modify this output directory if you prefer
- The setup script will also download the openweathermap icons to ./assets/icons/
- If you would prefer not to have the icons, you can run setup.sh with --noicons
- Assuming the path to the symbolic link is in your PATH, you should be able to run the program from anywhere on your system as the current user

# Features
- Get information about temperature, wind chill, weather, precipitation, and more
- Select location by latitude/longitude, city/state/country, or country/zipcode
- It is recommended to use latitude/longitude as the other options require a second API call to get geolocation data
- Choose options via command line arguments, or set environment variables via a .env file
- Command line options take precedence over environment variables
- Order of precedence for location selection is latitude/longitude > city/state/country > country/zipcode

```
# ./.env

API_KEY="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
UNITS="M" # M = metric, I = imperial
LATITUDE="..."
LONGITUDE="..."
CITY="..."
STATE="..."
COUNTRY="..."
ZIPCODE="..."
```

# Getting Access
- You will need an openweathermap.org API key to use this program
- Note that this program only supports the free tier of the API (current weather and five day forecast)
- Head to https://openweathermap.org, and create an account
- You should automatically be provided an API key upon creation of your account
- You can find your key(s) at https://home.openweathermap.org/api_keys
- It can take up to 2 hours upon creation of your account for your API key to get registered
- You can add your API key as an environment variable (as seen above), or pass it as a command line argument

```
owcli --key "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" --print temp
```

# Plans
- Only current weather is supported right now, but I plan to get the five day forecast supported as well
- Aside from being able to print individual data points, I would like to implement some nice way of summarizing important data
- I plan on using this for my eww bar/dashboard, and will likely include that setup as an example once I get around to setting it up
- I also intend to improve the documentation, and in particular the visibility of available options, as that is certainly not ideal at this time
- For the time being, the best way to find the list of printable options is to look at the print function within ./src/data/current_weather.rs

# Examples

```
# (Assumes you have your api key, latitude, and longitude set up as environment variables)

# print the current temperature
owcli --print temp

# print the current wind chill 
owcli --print feels_like

# print the current wind speed
owcli --print wind_speed

# print the current weather description
owcli --print description
```

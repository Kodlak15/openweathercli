# OpenWeatherCLI
This is a simple command line interface for the openweathermap.org API (free tier). You can use this in a terminal or in your scripts to get weather data for locations all around the world

# Getting Access
- You will need an openweathermap.org API key to use this program
- Head to https://openweathermap.org, and create an account
- You should automatically be provided an API key upon creation of your account
- You can find your key(s) at https://home.openweathermap.org/api_keys
- It can take up to 2 hours upon creation of your account for your API key to get registered
- You can add your API key as an environment variable or pass it as a command line argument

# Configuration
If you prefer, you can configure the program using a yaml file read from $HOME/.config/owcli/config.yaml. An example configuration file is included. 

# TODO
- Add support for five day forecast
- Improve documentation
- Allow configuration file to be loaded from alternate locations

# Examples
```
# print the current temperature
owcli --print temp

# print the current wind chill 
owcli --print feels_like

# print the current wind speed
owcli --print wind_speed

# print the current weather description
owcli --print description
```

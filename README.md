# OpenWeatherCLI
- A command line interface for the openweathermap.org API (free tier)
- Get weather data for locations around the world from the command line

# Getting Access
- You will need an openweathermap.org API key to use this program
- Head to https://openweathermap.org, and create an account
- You should automatically be provided an API key upon creation of your account
- You can find your key(s) at https://home.openweathermap.org/api_keys
- It can take up to 2 hours upon creation of your account for your API key to get registered
- You can add your API key as an environment variable or pass it as a command line argument

# TODO
- Add support for five day forecast
- Improve documentation

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

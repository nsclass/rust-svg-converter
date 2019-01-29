@app_setting_api
Feature: Application Settings API
  Scenario: This will verify application settings API

    When I send GET "/api/v1/settings"
    And I expect HTTP status code is 200
    And I expect that the JSON include:
    """
      {
        "appTokenSettings": {
          "expireInSeconds": 7200
        },
        "appSvgSettings": {
          "useLimit": true,
          "numberOfColorLimitation": 16,
          "imageSizeLimitation": 2097152
        }
      }
    """

    When I send PUT "/api/v1/settings/svgSettings" with json
    """
    {
        "useLimit": false,
        "numberOfColorLimitation": 256,
        "imageSizeLimitation": 5242880
    }
    """
    And I expect HTTP status code is 200
    And I expect that the JSON include:
    """
      {
        "useLimit": false,
        "numberOfColorLimitation": 256,
        "imageSizeLimitation": 5242880
      }
    """


@app_setting_api
Feature: Application Settings API
  Scenario: This will verify application settings API
    When I login /api/v1/login with "read@user.com:password"
    And I expect HTTP status code is 200

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

    When I login /api/v1/login with "admin@admin.com:password"
    And I expect HTTP status code is 200
    When I send PUT "/api/v1/settings/tokenSettings" with json
    """
    {
       "expireInSeconds": 720
    }
    """
    And I expect HTTP status code is 200
    And I expect that the JSON include:
    """
    {
       "expireInSeconds": 720
    }
    """


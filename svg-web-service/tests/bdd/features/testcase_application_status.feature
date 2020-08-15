# Copyright (c) 2020 Nam Seob Seo
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

@application_status
Feature: application health check and status API

  Scenario: Application status
    When I send GET "/health"
    And I expect HTTP status code is 200

    And the JSON response should be:
      """
      {
        "status": "UP"
      }
      """

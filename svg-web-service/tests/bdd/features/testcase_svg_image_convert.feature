@svg_image_converter
Feature: SVG image converter
  @create_svg_successfully
  Scenario: Verifying SVG image conversion

    Given I keep the value at "IMAGE_DATA" from a file "features/samples/image_sample_base64.txt"

    When I send PUT "/svg/conversion" with json
    """
    {
      "image_file_name" : "image_sample_base64.jpg",
      "number_of_colors" : 16,
      "image_base64_data": %{IMAGE_DATA}
    }
    """
    And I expect HTTP status code is 200
    And I expect that the JSON include:
    """
    {
       "image_file_name": "image_sample_base64.jpg"
    }
    """


#  @create_svg_failure_case
#  Scenario: Verifying SVG image conversion
#
#    Given I keep the value at "IMAGE_DATA" from a file "features/samples/image_sample_base64.txt"
#
#    When I send PUT "/api/v1/svg/conversion" with json
#    """
#    {
#      "imageFilename" : "image_sample_base64.jpg",
#      "imageDataBase64": %{IMAGE_DATA}
#    }
#    """
#    And I expect HTTP status code is 400
#    Then I expect HTTP JSON error message contains text
#    """
#    Not supported image size
#    """


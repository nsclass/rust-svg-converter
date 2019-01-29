
When(/^I send (GET|POST|PUT|PATCH) "(.+)" without token$/) do |method, url|
  request_http_with_token(method, url, get_bearer_token)
end

When(/^I send (GET|POST|PUT|PATCH) "(.+)"$/) do |method, url|
  request_http_with_token(method, url, get_bearer_token)
end

When(/^I login (.+) with "(.+):(.+)"$/) do |url, username, password|
  request_http_login(username, password, url)
end


When(/^I send (GET|POST|PUT|PATCH) "(.+)" with (json|text|form)$/) do |method, url, content_type, content|
  request_http_with_token(method, url, get_bearer_token, content_type, content)
end

When(/^I send (GET|POST|PUT|PATCH) "(.+)" with (json|text|form) without token$/) do |method, url, content_type, content|
  request_http_with_token(method, url, nil, content_type, content)
end

When(/^I register a user "(.+)" with email: "(.+)" and password: "(.+)"$/) do |url, email, password|
  insert_test_user(url, email, password)
end

When (/^I wait for (\d+) seconds$/) do |wait|
  sleep(wait)
end

And (/^I expect HTTP status code is (\d+)$/) do |code|

  result_code = last_json.code
  begin
    result_code.should == code
  rescue RSpec::Expectations::ExpectationNotMetError => error
    # better error msg with actual JSON vs just the diff from json_spec
    full_error = "#{error}\nHTTP response: #{$http_json_response.body}\n#{$http_json_response.headers}"
    raise RSpec::Expectations::ExpectationNotMetError.new(full_error)
  end
end

Then(/^I expect HTTP JSON error message contains text$/) do |string|
  response_json = JSON.parse($http_json_response.body)
  error_message = response_json['message']

  begin
    error_message.should include(string)
  rescue RSpec::Expectations::ExpectationNotMetError => json_error
    # better error msg with actual JSON vs just the diff from json_spec
    full_error = "Actual full JSON:\n" + $http_json_response.body +
        "\n--------Text Not matched ----------\n" + error_message + "\n"
    raise RSpec::Expectations::ExpectationNotMetError.new(full_error)
  end
end

Then(/^I expect HTTP JSON field (.+) contains text$/) do |field, string|
  response_json = JSON.parse($http_json_response.body)
  json_field = response_json[field]

  puts json_field

  begin
    json_field.should include(string)
  rescue RSpec::Expectations::ExpectationNotMetError => json_error
    # better error msg with actual JSON vs just the diff from json_spec
    full_error = "Actual full JSON:\n" + $http_json_response.body +
        "\n--------Text Not matched ----------\n" + json_field + "\n"
    raise RSpec::Expectations::ExpectationNotMetError.new(full_error)
  end
end

Then(/^I expect HTTP JSON field (.+) matches to$/) do |field, string|
  response_json = JSON.parse($http_json_response.body)
  json_field = response_json[field]

  puts json_field

  begin
    json_field.match(string)
  rescue RSpec::Expectations::ExpectationNotMetError => json_error
    # better error msg with actual JSON vs just the diff from json_spec
    full_error = "Actual full JSON:\n" + $http_json_response.body +
        "\n--------Text Not matched ----------\n" + json_field + "\n"
    raise RSpec::Expectations::ExpectationNotMetError.new(full_error)
  end
end

Then(/^I expect JSON result to, except fields "(.*?)"$/) do |except_fields, string|
  string = replace_memorized_variables(string,false)
  expected = JSON.parse(string)

  except_field_names = except_fields.gsub(/\s+/, "").split(",").to_set
  expected_result = remove_field_names_exclusive(expected, except_field_names)

  expected_field_names = get_unique_field_names(expected_result)

  actual = JSON.parse(last_json())
  actual_result = remove_field_names_exclusive(actual, except_field_names)
  actual_compare = remove_field_names(actual_result, expected_field_names)

  # use json_spec comparison with diff built in
  begin
    JSON.dump(actual_compare).should be_json_eql(JSON.dump(expected_result))
  rescue RSpec::Expectations::ExpectationNotMetError => json_error
    # better error msg with actual JSON vs just the diff from json_spec
    full_error = "Actual full JSON:\n" + JSON.pretty_generate(actual) +
        "\n------------------------------\nActual JSON with expected fields only:\n" + JSON.pretty_generate(actual_compare) +
        "\n------------------------------\n" + json_error.to_s
    raise RSpec::Expectations::ExpectationNotMetError.new(full_error)
  end

end

Then(/^I expect JSON result to$/) do |string|
  begin
    last_json.should be_json_eql(string)
  rescue RSpec::Expectations::ExpectationNotMetError => json_error
    # better error msg with actual JSON vs just the diff from json_spec
    full_error = "Actual JSON:\n" + JSON.pretty_generate(last_json) + "\n" + json_error.to_s

    raise RSpec::Expectations::ExpectationNotMetError.new(full_error)
  end
end

Then(/^I expect that the JSON include:$/) do |string|
  string = replace_memorized_variables(string,false)
  expected = JSON.parse(string)
  expected_field_names = get_unique_field_names(expected)

  actual = JSON.parse(last_json())
  actual_compare = remove_field_names(JSON.parse(last_json()), expected_field_names)

  # use json_spec comparison with diff built in
  begin
    JSON.dump(actual_compare).should be_json_eql(string)
  rescue RSpec::Expectations::ExpectationNotMetError => json_error
    # better error msg with actual JSON vs just the diff from json_spec
    full_error = "Actual full JSON:\n" + JSON.pretty_generate(actual) +
        "\n------------------------------\nActual JSON with expected fields only:\n" + JSON.pretty_generate(actual_compare) +
        "\n------------------------------\n" + json_error.to_s
    raise RSpec::Expectations::ExpectationNotMetError.new(full_error)
  end
end

Given(/^I keep the value at "(.+)" from a file "(.+)"$/) do |json_key, filename|
  data = File.read(filename)
  JsonSpec.memorize(json_key, '"' + data + '"')
end

Given(/^I keep the JSONVAR at "(.+)" with values$/) do |json_key, json_value|
  value = replace_memorized_variables(json_value, false)
  JsonSpec.memorize(json_key, value)
end

Given(/^I update the JSONVAR at "(.+)" with values$/) do |json_key, json_values|
  memorized_json = JSON.parse(JsonSpec.memory[json_key.to_sym])
  updated_json_values = JSON.parse(json_values)

  updated_json_values.each do |key, value|
    memorized_json[key] = value
  end

  JsonSpec.memory[json_key.to_sym] = JSON.dump(memorized_json)
end


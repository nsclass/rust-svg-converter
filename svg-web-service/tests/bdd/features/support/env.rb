# Copyright (c) 2020 Nam Seob Seo
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

require 'json'
require 'net/http'
require 'uri'
require 'yaml'
require 'rest-client'
require 'json_spec/cucumber'
require 'rspec/expectations'

puts RUBY_VERSION

AfterConfiguration do |config|
  $config = load_environment('config_bdd.yml')
  wait_for_main_app_start
end

Before do
  clear_in_memory_data
end

After do

end

def clear_in_memory_data
  # clear data when using memory based dao

end

def clear_token_data
  $http_json_response = nil
  $last_login_token = nil
end

def load_environment(file)
  YAML.load_file(file)
end

def wait_for_main_app_start

  (1..120).each {|idx|
    puts "Waiting for svg-web-service to start(retry: #{idx}) ..."

    request_http_with_token('GET', '/health')
    app_status = JSON.parse(last_json.body)
    puts app_status
    if app_status['status'] == 'UP'
      puts 'svg-web-service is up and running'
      break
    end

    sleep(1.0)
  }
end

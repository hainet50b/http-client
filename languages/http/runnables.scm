((request
  method: (method) @method @run
  url: (target_url) @url
  [(json_body) (xml_body) (raw_body)]? @body) @_request
  (#set! tag http-request))

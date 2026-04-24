((request
  method: (method) @method @run
  url: (target_url) @url
  (json_body)? @body) @_request
  (#set! tag http-request))

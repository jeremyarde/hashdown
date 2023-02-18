# Market research

## Typeform

Form Types
1. application
2. checklist
3. contact
4. feedback
5. lead gen
6. order form
7. payment
8. poll
9. quiz
10. registration form
11. request
12. research
13. other


Features
1. Logic/Branching
2. quiz/scoring
3. integrations
   1. slack
   2. hubspot
   3. google sheets
   4. excel
4. shoes progress at top of form
5. Adds a "create typeform" button at the end
6. results
   1. insights
      1. views
      2. starts
      3. submissions
      4. completion rate
      5. time to complete
      6. question drop off rate
   2. summary
      1. 
   3. responses
      1. filter, search, categorize (tags)

question types
1. text (short, long)
2. email
3. multi
4. statement
5. payment
6. legal
7. contact info
   1. address
   2. contact info
   3. email
   4. phone
   5. website
8. Date/schedule
   1. calendly
   2. date
9. File upload
10. rating
    1.  matrix
    2.  net promoter score
    3.  opinion scale
    4.  ranking
    5.  rating
11. number
12. choices
    1.  dropdown
    2.  multichoice
    3.  picture choice
    4.  yes/no
13. form structure
    1.  question group
    2.  statement
    3.  end
    4.  redirect
    5.  welcome


Plans
1. biz 
   1. $83
   2. 10k - 50k responses
   3. 5 users
2. plus
   1. 50 usd
   2. 1k responses
   3. 3 users
3. basic
   1. 25 usd
   2. 100 responses
4. free

- pay per responses and plan type
- users different
- biz
  - 50k, 4000 usd
  - 25k, 1500 usd
  - 15k, 500 usd
  - 10k
- basic
  - 750, 195 usd


Tech details
- /forms/{id}/start-submission
  - request
    - {"visit_response_id":"CR2191dXFa9S"}
  - response
    - {"signature":"20906d686f70356f35656c6c6164687562677535776d686f70356f357071616b636c34313339363936363463366336363533373434333536343134353337366237613339343233303664373236613737353935613431363436393336343833313336333733363336333133363335333433383135353032396564383237633432666462646462653531306439313237633062316436333730363739656366663266613162633234343563653533353535663631363736363136353438","submission":{"response_id":"mhop5o5elladhubgu5wmhop5o5pqakcl","type":"started","form_id":"U37VdiXX","landed_at":1676616548,"visit_response_id":"CR2191dXFa9S","metadata":{"user_agent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/109.0","platform":"other","referer":"https://59x90aj6oxz.typeform.com/to/U37VdiXX","network_id":"2ee3d5a245","ip":"207.244.142.79","browser":"default","client":"stakhanov","id_type":"form-id","source":"","medium":"","medium_version":"","embed_trigger_type":"","domain_type":"standard","subdomain_type":"custom"}}}
- /forms/{id}/insights/events/v3/see
  - request
    - form_id=U37VdiXX&field_id=c2wmUoyJL91v&previous_seen_field_id=XUBWo6zbAUeY&response_id=CR2191dXFa9S&user_agent=Mozilla%2F5.0%20(Macintosh%3B%20Intel%20Mac%20OS%20X%2010.15%3B%20rv%3A109.0)%20Gecko%2F20100101%20Firefox%2F109.0&version=1
  - response = None
- https://59x90aj6oxz.typeform.com/forms/U37VdiXX/insights/events/v3/see
  - request
    - form_id=U37VdiXX&field_id=f7LpYTJowBDN&previous_seen_field_id=c2wmUoyJL91v&response_id=CR2191dXFa9S&user_agent=Mozilla%2F5.0%20(Macintosh%3B%20Intel%20Mac%20OS%20X%2010.15%3B%20rv%3A109.0)%20Gecko%2F20100101%20Firefox%2F109.0&version=1
  - response
    - none
- https://59x90aj6oxz.typeform.com/forms/U37VdiXX/complete-submission
  - request
    - {"signature":"20906d686f70356f35656c6c6164687562677535776d686f70356f357071616b636c34313339363936363463366336363533373434333536343134353337366237613339343233303664373236613737353935613431363436393336343833313336333733363336333133363335333433383135353032396564383237633432666462646462653531306439313237633062316436333730363739656366663266613162633234343563653533353535663631363736363136353438","form_id":"U37VdiXX","landed_at":1676616548,"answers":[{"field":{"id":"XUBWo6zbAUeY","type":"short_text"},"type":"text","text":"testing"},{"field":{"id":"c2wmUoyJL91v","type":"multiple_choice"},"type":"choices","choices":[{"id":"YWpn4l8aOCzD","label":"Terrific!"}]},{"field":{"id":"0DINhphoifsc","type":"ranking"},"type":"choices","choices":[{"id":"MSZZ7grfALiF","label":"choice 1"}]}],"thankyou_screen_ref":"01GS3HPH9WNEWHH9SGCTHQ1346"}
  - response
    - {"response_id":"mhop5o5elladhubgu5wmhop5o5pqakcl","type":"completed","form_id":"U37VdiXX","landed_at":1676616548,"submitted_at":1676616593,"metadata":{"user_agent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/109.0","platform":"other","referer":"https://59x90aj6oxz.typeform.com/to/U37VdiXX","network_id":"2ee3d5a245","ip":"207.244.142.79","browser":"default","client":"stakhanov","id_type":"form-id","source":"","medium":"","medium_version":"","embed_trigger_type":"","domain_type":"standard","subdomain_type":"custom"},"answers":[{"type":"text","field":{"type":"short_text","id":"XUBWo6zbAUeY"},"text":"testing"},{"type":"choices","field":{"type":"multiple_choice","id":"c2wmUoyJL91v"},"choices":[{"id":"YWpn4l8aOCzD","label":"Terrific!","ref":"01GS3HPH9W00W8GBKARYB1ZACP"}]},{"type":"choices","field":{"type":"ranking","id":"0DINhphoifsc"},"choices":[{"id":"MSZZ7grfALiF","label":"choice 1","ref":"6574f825-1826-48e0-89a9-c16d27654328"}]}],"thankyou_screen_ref":"01GS3HPH9WNEWHH9SGCTHQ1346"}
- https://59x90aj6oxz.typeform.com/forms/U37VdiXX/insights/events/v3/see
  - request
    - form_id=U37VdiXX&field_id=EndingID&previous_seen_field_id=0DINhphoifsc&response_id=CR2191dXFa9S&user_agent=Mozilla%2F5.0%20(Macintosh%3B%20Intel%20Mac%20OS%20X%2010.15%3B%20rv%3A109.0)%20Gecko%2F20100101%20Firefox%2F109.0&version=1

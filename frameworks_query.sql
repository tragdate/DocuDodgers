WITH LanguageMapping AS (
    SELECT
    CASE
    WHEN TagName IN ('angular', 'angularjs', 'angular2+', 'angular2', 'angular4', 'angular5', 'angular6', 'angular7', 'angular8', 'angular9', 'angular10', 'angular-directive', 'angular-service', 'angular-components', 'angular-cli', 'angular-forms', 'angular-routing', 'angular-pipe', 'angular-observables', 'ngrx', 'angular-http', 'angular-material') THEN 'Angular'
    WHEN TagName IN ('aurelia', 'aurelia-binding', 'aurelia-cli', 'aurelia-framework', 'aurelia-router') THEN 'Aurelia'
    WHEN TagName IN ('backbone.js', 'marionette.js') THEN 'Backbone'
    WHEN TagName IN ('d3.js', 'c3.js', 'nvd3', 'dc.js', 'three.js', 'echarts', 'chart.js') THEN 'D3/Three.js'
    WHEN TagName IN ('ember.js', 'ember-data', 'ember-cli', 'ember-addon', 'ember-engines', 'glimmer.js') THEN 'Ember.js'
    WHEN TagName IN ('jquery', 'jquery-ui', 'jquery-mobile', 'jquery-select2', 'jquery-datatables', 'jquery-validate') THEN 'jQuery'
    WHEN TagName IN ('node.js', 'express.js', 'koa', 'socket.io', 'sails.js', 'meteor', 'nest.js', 'hapi', 'adonisjs', 'fastify') THEN 'Node.js'
    WHEN TagName IN ('polymer', 'lit-element', 'lit-html') THEN 'Polymer'
    WHEN TagName IN ('reactjs', 'react-native', 'react-router', 'redux', 'react-redux', 'react-context', 'react-hooks', 'react-apollo', 'react-component', 'react-router-dom', 'react-hook-form', 'react-query', 'next.js', 'gatsby', 'cra', 'flux', 'mobx') THEN 'React'
    WHEN TagName IN ('svelte', 'sapper', 'svlete-kit') THEN 'Svelte'
    WHEN TagName IN ('typescript', 'ts', 'ionic-framework', 'ionic', 'ionic2', 'ionic3', 'ionic4', 'ionic5') THEN 'Ionic/TypeScript'
    WHEN TagName IN ('vue.js', 'vuetify', 'vuex', 'vue-router', 'vue-component', 'vue-cli', 'vue-directive', 'nuxt.js', 'nuxt', 'quasar-framework', 'vuepress', 'pinia') THEN 'Vue.js'
    ELSE NULL
    END AS Framework,
    PostId
    FROM (
        SELECT PostId, TagName,
        ROW_NUMBER() OVER (PARTITION BY PostId ORDER BY TagName) as rn
        FROM PostTags
        INNER JOIN Tags ON Tags.Id = PostTags.TagId
    ) t
    WHERE rn = 1
)
SELECT
DATEPART(yyyy, Posts.CreationDate) AS Year,
DATEPART(mm, Posts.CreationDate) AS Month,
LanguageMapping.Framework,
COUNT(*) AS NumQuestions
FROM
Posts
INNER JOIN
LanguageMapping ON Posts.Id = LanguageMapping.PostId
WHERE
Posts.PostTypeId = 1
AND LanguageMapping.Framework IS NOT NULL
GROUP BY
DATEPART(yyyy, Posts.CreationDate),
DATEPART(mm, Posts.CreationDate),
LanguageMapping.Framework
ORDER BY
Year, Month, NumQuestions DESC;

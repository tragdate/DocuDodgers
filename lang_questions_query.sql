WITH LanguageMapping AS (
    SELECT
    CASE
    WHEN TagName IN ('js', 'javascript', 'jquery', 'node.js', 'reactjs', 'angularjs', 'vue.js', 'd3.js', 'express.js', 'ecmascript', 'vanilla-js', 'react', 'angular', 'vue') THEN 'JS'
    WHEN TagName IN ('html', 'html5', 'css', 'css3', 'sass', 'bootstrap') THEN 'HTML/CSS'
    WHEN TagName IN ('sql', 'mysql', 'postgresql', 'oracle', 'sql-server', 'sqlite', 'plsql', 'tsql') THEN 'SQL'
    WHEN TagName IN ('python', 'python-2.x', 'python-3.x', 'django', 'flask', 'pandas', 'numpy', 'scipy') THEN 'Python'
    WHEN TagName IN ('java', 'java-8', 'java-11', 'spring','maven', 'gradle') THEN 'Java'
    WHEN TagName IN ('bash', 'shell', 'zsh', 'ksh', 'csh', 'tcsh', 'dash') THEN 'Shell'
    WHEN TagName IN ('c#', 'csharp', '.net', 'asp.net', 'entity-framework', 'linq') THEN 'C#'
    WHEN TagName IN ('php', 'php7','php8', 'laravel', 'symfony', 'codeigniter', 'zend-framework') THEN 'PHP'
    WHEN TagName IN ('typescript', 'ts', 'angular', 'ionic-framework') THEN 'TS'
    WHEN TagName IN ('c++', 'cxx', 'cpp', 'boost', 'stl') THEN 'C++'
    WHEN TagName IN ('c', 'gcc', 'glibc', 'libc') THEN 'C'
    WHEN TagName IN ('go', 'golang', 'go-language') THEN 'Go'
    WHEN TagName IN ('kotlin', 'android', 'spring-boot') THEN 'Kotlin'
    WHEN TagName IN ('ruby', 'rails', 'jruby', 'sinatra') THEN 'Ruby'
    WHEN TagName IN ('swift', 'ios', 'xcode', 'cocoa') THEN 'Swift'
    WHEN TagName IN ('r', 'rstudio', 'shiny', 'ggplot2') THEN 'R'
    WHEN TagName IN ('objective-c', 'cocoa-touch', 'core-data', 'cocoa') THEN 'Obj-C'
    WHEN TagName IN ('scala', 'akka', 'playframework', 'sbt') THEN 'Scala'
    WHEN TagName IN ('rust', 'rust-cargo') THEN 'Rust'
    WHEN TagName IN ('lisp', 'elisp', 'common-lisp', 'scheme', 'clojure', 'racket') THEN 'Lisp'
    WHEN TagName IN ('powershell') THEN 'PowerShell'
    WHEN TagName IN ('perl', 'perl5', 'perl6', 'cpan') THEN 'Perl'
    WHEN TagName IN ('assembly', 'asm', 'x86', 'mips') THEN 'Assembly'
    WHEN TagName IN ('groovy', 'grails', 'gradle') THEN 'Groovy'
    WHEN TagName IN ('elixir', 'phoenix', 'erlang') THEN 'Elixir'
    WHEN TagName IN ('clojure', 'leiningen', 'ring') THEN 'Clojure'
    WHEN TagName IN ('haskell', 'ghc', 'cabal', 'stack') THEN 'Haskell'
    WHEN TagName IN ('lua', 'luajit', 'corona', 'love2d') THEN 'Lua'
    WHEN TagName IN ('matlab', 'octave', 'simulink') THEN 'MATLAB'
    WHEN TagName IN ('dart', 'flutter') THEN 'Dart'
    ELSE NULL
    END AS ProgrammingLanguage,
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
LanguageMapping.ProgrammingLanguage,
COUNT(*) AS NumQuestions
FROM
Posts
INNER JOIN
LanguageMapping ON Posts.Id = LanguageMapping.PostId
WHERE
Posts.PostTypeId = 1 
AND LanguageMapping.ProgrammingLanguage IS NOT NULL 
GROUP BY
DATEPART(yyyy, Posts.CreationDate),
DATEPART(mm, Posts.CreationDate),
LanguageMapping.ProgrammingLanguage
ORDER BY
Year, Month, NumQuestions DESC

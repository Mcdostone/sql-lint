<h1 align="center">SQL-lint</h1>

<p align="center">
  <a href="https://github.com/Mcdostone/sql-lint/actions"><img src="https://github.com/Mcdostone/sql-lint/workflows/Cargo%20test/badge.svg" alt="Build Status"></a>
  <a href="https://deps.rs/repo/github/Mcdostone/sql-lint"><img src="https://deps.rs/repo/github/Mcdostone/sql-lint/status.svg" alt="Dependency status"></a>
  <a href="https://app.netlify.com/sites/laughing-euclid-cf6cad/deploys"><img src="https://api.netlify.com/api/v1/badges/6a46eb76-2ffb-4409-b554-5a0782da01c4/deploy-status" alt="Netlify Status"></a>
</p>

<p align="center">Experimental SQL formatter written in rust.</p>

**sql-lint** is an experimental implementation of the [SQL Style Guide](https://www.sqlstyle.guide/) written by Simon Holywell.
  Parsing step is mainly based on the [ANSI SQL 2016 grammar](https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html). This project is just a way for me to write my first lines of rust and try webAssembly.


## Usage
```bash
git clone git@github.com:Mcdostone/sql-lint.git
cargo install --path ./sql-lint

sql-lint "SELECT CASE postcode WHEN 'BN1' THEN 'Brighton' WHEN 'EH1' THEN 'Edinburgh' END AS city FROM office_locations WHERE country = 'United Kingdom' AND opening_time BETWEEN 8 AND 9 AND postcode IN ('EH1', 'BN1', 'NN1', 'KW1');"

echo "UPDATE file_system SET file_modified_date = '1980-02-22 13:19:01.00000',file_size = 209732;" | sql-lint
```


## Resources
 - SQL grammar, https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html
 - sequel fumpt, https://sqlfum.pt/
 - nom parser, https://github.com/Geal/nom
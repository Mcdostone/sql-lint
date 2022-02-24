import "./style.css";
import init, { format_sql as format } from "sql-lint";

const DEFAULT_QUERY = `SELECT f.species_name,
AVG(f.height) AS average_height, AVG(f.diameter) AS average_diameter
FROM flora AS f
WHERE f.species_name = 'Banksia'
OR f.species_name = 'Sheoak'
OR f.species_name = 'Wattle'
GROUP BY f.species_name, f.observation_date;
`;

async function main() {
  await init();
  const editor = document.querySelector("textarea") as HTMLTextAreaElement;
  const formatButton = document.querySelector("button") as HTMLButtonElement;
  const error = document.querySelector(".error") as HTMLElement;
  const url = new URL(document.location.toString());
  const q = url.searchParams.get("q") || "";
  if (q !== "") {
    editor.value = q;
  } else {
    editor.value = DEFAULT_QUERY;
  }
  formatButton.addEventListener("click", (e) => {
    e.preventDefault();
    error.innerHTML = "";
    url.searchParams.set("q", editor.value);
    history.pushState({}, editor.value, url);
    try {
      editor.value = format(editor.value);
    } catch (err) {
      error.innerHTML = err.toString();
    }
  });
}

main();

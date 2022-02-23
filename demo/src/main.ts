import "./style.css"
import init, {format_sql as format} from "sql-lint";

async function main() {
  await init();
  const editor = document.querySelector("textarea") as HTMLTextAreaElement
  const formatButton = document.querySelector("button") as HTMLButtonElement
  const error = document.querySelector(".error") as HTMLElement
  const url = new URL(document.location.toString())
  const q = url.searchParams.get("q") || ""
  if(q !== "") {
    editor.value = q
  }
  formatButton.addEventListener('click', e => {
    e.preventDefault()
    error.innerHTML = ''
    try {
      editor.value = format(editor.value)
    } catch(err) {
      error.innerHTML = err.toString()
    }
    url.searchParams.set("q", editor.value)
    history.pushState({}, editor.value, url);
  })
}

main()

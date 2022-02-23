import "./style.css"
import init, {format_sql as format} from "sql-lint";

async function main() {
  await init();
  const editor = document.querySelector("textarea") as HTMLTextAreaElement
  const formatButton = document.querySelector("button") as HTMLButtonElement
  const error = document.querySelector(".error") as HTMLElement
  formatButton.addEventListener('click', e => {
    e.preventDefault()
    error.innerHTML = ''
    try {
      editor.value = format(editor.value)
    } catch(err) {
      error.innerHTML = err.toString()
    }
  })
}

main()

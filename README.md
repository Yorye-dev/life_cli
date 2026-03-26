# life_cli

CLI local-first para gestionar una carpeta `life` en texto plano.

## Principios

- editable con Vim
- `index.md` como hub principal
- `calendar.txt` solo muestra hoy + 7 días
- los TODO aparecen arriba del día
- los eventos de todo el día van debajo
- los eventos con hora van ordenados de menor a mayor

## Estructura

- `todo.txt`
- `calendar.txt`
- `events.txt`
- `recurring.txt`
- `historialTodo.txt`
- `historialCalendar.txt`
- `index.md`
- `notes/`

## Comandos iniciales

```bash
cargo run -- init
cargo run -- todo add "(A) 2026-03-26 #h llamar a juan"
cargo run -- calendar refresh



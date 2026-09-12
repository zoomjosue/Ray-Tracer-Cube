# Ray tracer: cubo verde oscuro

## Controles

- `←` / `→`: orbitar alrededor del cubo.
- `↑` / `↓`: subir o bajar la cámara sobre la órbita.
- `Escape`: cerrar la ventana.

## Ejecutar

```bash
cargo run
```

La intersección del cubo se resuelve con el método de slabs para una caja
alineada a los ejes. La cámara conserva la transformación de base y el
movimiento orbital usados por el proyecto de referencia.

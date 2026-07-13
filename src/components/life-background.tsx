import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { StyleSheet, View, type LayoutChangeEvent } from "react-native";
import Svg, { Circle } from "react-native-svg";

interface GridSize {
  columns: number;
  rows: number;
}

function seededCells(size: GridSize): boolean[] {
  const length = size.columns * size.rows;
  return Array.from({ length }, (_, index) => {
    let value = (index * 2654435761 + size.columns * 977 + size.rows * 131) >>> 0;
    value ^= value >>> 16;
    return value % 7 === 0;
  });
}

export function stepLife(cells: boolean[], size: GridSize): boolean[] {
  const next = new Array<boolean>(cells.length).fill(false);
  const at = (x: number, y: number) => cells[y * size.columns + x] ?? false;
  for (let y = 0; y < size.rows; y += 1) {
    for (let x = 0; x < size.columns; x += 1) {
      let neighbors = 0;
      for (let dy = -1; dy <= 1; dy += 1) {
        for (let dx = -1; dx <= 1; dx += 1) {
          if (dx === 0 && dy === 0) continue;
          const nx = x + dx;
          const ny = y + dy;
          if (nx >= 0 && nx < size.columns && ny >= 0 && ny < size.rows) {
            neighbors += Number(at(nx, ny));
          }
        }
      }
      const alive = at(x, y);
      next[y * size.columns + x] = neighbors === 3 || (alive && neighbors === 2);
    }
  }
  return next;
}

export const LifeBackground = memo(function LifeBackground({
  color,
  subtle,
}: {
  color: string;
  subtle: boolean;
}) {
  const cellSize = subtle ? 24 : 18;
  const [dimensions, setDimensions] = useState({ width: 0, height: 0 });
  const size = useMemo<GridSize>(
    () => ({
      columns: Math.max(1, Math.ceil(dimensions.width / cellSize)),
      rows: Math.max(1, Math.ceil(dimensions.height / cellSize)),
    }),
    [cellSize, dimensions],
  );
  const [cells, setCells] = useState(() => seededCells(size));
  const sizeKey = `${size.columns}x${size.rows}`;
  const previousSizeKey = useRef(sizeKey);

  useEffect(() => {
    if (previousSizeKey.current !== sizeKey) {
      previousSizeKey.current = sizeKey;
      setCells(seededCells(size));
    }
  }, [size, sizeKey]);

  useEffect(() => {
    const timer = setInterval(
      () => setCells((current) => stepLife(current, size)),
      subtle ? 1200 : 650,
    );
    return () => clearInterval(timer);
  }, [size, subtle]);

  const onLayout = useCallback((event: LayoutChangeEvent) => {
    const { width, height } = event.nativeEvent.layout;
    setDimensions({ width, height });
  }, []);

  return (
    <View pointerEvents="none" style={StyleSheet.absoluteFill} onLayout={onLayout}>
      {dimensions.width > 0 ? (
        <Svg width="100%" height="100%">
          {cells.map((alive, index) =>
            alive ? (
              <Circle
                key={index}
                cx={(index % size.columns) * cellSize + cellSize / 2}
                cy={Math.floor(index / size.columns) * cellSize + cellSize / 2}
                fill={color}
                opacity={subtle ? 0.2 : 0.32}
                r={1.25}
              />
            ) : null,
          )}
        </Svg>
      ) : null}
    </View>
  );
});

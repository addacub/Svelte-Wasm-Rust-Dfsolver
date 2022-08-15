/**
 * Automatically calculates the stroke lightness based on input lightness
 * @param lightness The fill lightness of the piece
 * @returns the stroke lightness of the internal borders
 */
export function getStrokeLightness(lightness: number): number {
    if (lightness < 65) {
        return lightness + 25;
    } else {
        return lightness - 25;
    }
}
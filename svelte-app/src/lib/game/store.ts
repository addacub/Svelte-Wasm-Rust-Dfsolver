import { writable, type Writable } from 'svelte/store';

// Variables used for the drag and drop game
export let drawScale: Writable<number> = writable(60);
export let lineWidth: Writable<number> = writable(0.05);
export let scaleFactor: Writable<number> = writable(0.8);
export let isSelectable: Writable<boolean> = writable(true)

/**
 * Is a cartesian coordinate
 * @param x The x-coordinate of the cartesian coordinate
 * @param y The y-coordinate of the cartesian coordinate
 */
export class Point {
    
    private x: number;
    private y: number;

    public constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    public getX(): number {
        return this.x;
    }

    public getY(): number {
        return this.y;
    }

}

export let mousePosition: Writable<Point> = writable(new Point(0, 0));

/**
 * Automatically calculates the stroke lightness based on input lightness
 * @param lightness The fill lightness of the piece
 * @returns the stroke lightness of the internal borders
 */
export function getStrokeLightness(lightness: number): number {
    if (lightness < 75) {
        return lightness + 25;
    } else {
        return lightness - 25;
    }
}

/**
 * Returns the slope of the line between two points.
 * @param p1 Point 1
 * @param p2 Point 2
 * @returns the slope of the line between the two points
 */
export function slope(p1: Point, p2: Point): number {
    return (p1.getY() - p2.getY()) / (p1.getX() - p2.getX())
}

/**
 * Returns the intercept of the line from a given slope and intercept.
 * @param point A point on the line
 * @param slope The slope of the line
 * @returns The intercept of the line
 */
export function intercept(point: Point, slope: number) {
    return point.getY() - slope * point.getX()
}

/**
 * Determines if the slope between two points are vertical or horizontal.
 * @param p1 Point 1
 * @param p2 Point 2
 * @returns True if the lines between the points have a slope.
 */
export function isVerticalOrHorizontalSlope(p1: Point, p2: Point): boolean {
    return (p1.getX() - p2.getX() < 1e-4 || p1.getY() - p2.getY() < 1e-4)
}
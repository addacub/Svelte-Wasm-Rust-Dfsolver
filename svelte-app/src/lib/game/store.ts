import { writable, type Writable } from 'svelte/store';

// Variables used for the drag and drop game
export let drawScale: Writable<number> = writable(60);
export let lineWidth: Writable<number> = writable(0.05);
export let scaleFactor: Writable<number> = writable(0.8);


/**
 * #0dc566 = hsl(149, 88, 41)
 * #dd33ff = hsl(290, 100, 60)
 * #e0d900 = hsl(58, 100, 44)
 * #5cffe9 = hsl(172, 100, 68)
 * #ff4775 = hsl(345,100,64)
 * #479dff = hsl(212,100,64)
 * #ff1f1f = hsl(0,100,56)
 * #97fe72 = hsl(104,99,72)
 */

/**
 * Is a cartesian coordinate
 * @param x The x-coordinate of the cartesian coordinate
 * @param y The y-coordinate of the cartesian coordinate
 */
export class Point {
    
    protected x: number;
    protected y: number;

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
 * Returns a point rotated around a provided point at the specified angle.
 * @param position The cartesian point to be rotated
 * @param origin The point which to rotate around
 * @param theta The angle of rotation in degrees as per standard unit circle. 
 * @returns 
 */
export function rotationAboutPoint(position: Point, origin: Point, theta: number): Point {
    // Convert from degrees to radians
    theta = (Math.PI / 180) * theta;
    
    // Translate so point of rotation is the origin
    let xTranslated = position.getX() - origin.getX();
    let yTranslated = position.getY() - origin.getY();
    
    // Rotate
    let xRotated = xTranslated * Math.cos(theta) - yTranslated * Math.sin(theta);
    let yRotated = yTranslated * Math.cos(theta) + xTranslated * Math.sin(theta);

    // Undo translation
    let x = xRotated + origin.getX();
    let y = yRotated + origin.getY();

    return new Point(x, y);
}

/**
 * Rotates and translates the canvas by the angle specified
 * @param theta the rotation angle, clockwise in radians
 */
 export function translateAndRotate(ctx: CanvasRenderingContext2D, centre: Point, theta: number): void {
    ctx.translate(centre.getX(), centre.getY());
    ctx.rotate((Math.PI / 180) * theta);
    ctx.translate(-centre.getX(), -centre.getY());
}

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
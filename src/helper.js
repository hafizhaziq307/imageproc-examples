export function isEmpty(x) {
    return [Object, Array].includes((x || {}).constructor) && !Object.entries((x || {})).length;
}
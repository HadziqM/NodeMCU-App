import { writable, derived,readable } from "svelte/store"


interface Notif{
  id:string,
  type:string,
  message:string,
  timeout:number
}


function createNotificationStore () {
  let _notifications= writable<Notif[]>([])
    function send (message:string, type = "default", timeout:number) {
        _notifications.update(state => {
            return [...state, { id: id(), type, message, timeout }]
        })
    }
    const notifications = derived(_notifications, $_notifications => {
        if ($_notifications.length > 0) {
            setTimeout(() => {
                _notifications.update(state => {
                    state.shift()
                    return state
                })
            }, $_notifications[0].timeout)
        }
        return $_notifications
    })

    const { subscribe } = notifications

    return {
        subscribe,
        send,
				default: (msg:string, timeout:number) => send(msg, "default", timeout),
        danger: (msg:string, timeout:number) => send(msg, "danger", timeout),
        warning: (msg:string, timeout:number) => send(msg, "warning", timeout),
        info: (msg:string, timeout:number) => send(msg, "info", timeout),
        success: (msg:string, timeout:number) => send(msg, "success", timeout),
    }
}

function id() {
    return '_' + new Date().getTime().toString()
};

export const notifications = createNotificationStore()

export const time = readable(new Date(), function start(set) {
	const interval = setInterval(() => {
		set(new Date());
	}, 1000);

	return function stop() {
		clearInterval(interval);
	};
});

const start = new Date();

export const elapsed = derived(
	time,
	$time => Math.round(($time.getTime() - start.getTime()) / 1000)
);

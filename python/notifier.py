import gi
gi.require_version('Notify','0.7')

from gi.repository import Notify
Notify.init("Test Notifier")

notification = Notify.Notification.new(
	"Hello!",
	"How\nare\n<b>you</b>?",
	"emblem-OK",
)

notification.set_urgency(1)


notification.show()

#notification.update("Summary","Message")
#notification.close()

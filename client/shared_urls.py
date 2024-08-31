import threading

class SharedUrls:
    def __init__(self):
        self.urls = []
        self.lock = threading.Lock()

    def get_urls(self):
        self.lock.acquire()
        result = self.urls.copy()
        self.lock.release()
        return result
    
    def set_urls(self, urls):
        self.lock.acquire()
        self.urls = urls
        self.lock.release()
package com.plugin.androidutils

import android.bluetooth.BluetoothManager
import android.content.Context
import android.content.Intent
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.drawable.BitmapDrawable
import android.graphics.drawable.Drawable
import android.net.wifi.WifiManager
import android.provider.Settings
import android.util.Base64
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.JSArray
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import android.app.Activity
import java.io.ByteArrayOutputStream

@TauriPlugin
class AndroidUtils(private val activity: Activity) : Plugin(activity) {

    @Command
    fun getInstalledApps(invoke: Invoke) {
        val pm: PackageManager = activity.packageManager
        val packages = pm.getInstalledApplications(PackageManager.GET_META_DATA)
        val result = JSArray()

        for (appInfo in packages) {
            val app = JSObject()
            app.put("packageName", appInfo.packageName)
            app.put("appName", pm.getApplicationLabel(appInfo).toString())
            app.put("isSystemApp", (appInfo.flags and ApplicationInfo.FLAG_SYSTEM) != 0)
            app.put("isUserApp", (appInfo.flags and ApplicationInfo.FLAG_SYSTEM) == 0)

            try {
                val pkgInfo = pm.getPackageInfo(appInfo.packageName, 0)
                app.put("versionName", pkgInfo.versionName ?: "")
                app.put("versionCode", pkgInfo.longVersionCode)
                app.put("firstInstallTime", pkgInfo.firstInstallTime)
                app.put("lastUpdateTime", pkgInfo.lastUpdateTime)
            } catch (e: PackageManager.NameNotFoundException) {
                app.put("versionName", "")
                app.put("versionCode", 0)
            }

            result.put(app)
        }

        val response = JSObject()
        response.put("apps", result)
        invoke.resolve(response)
    }

    @Command
    fun getUserInstalledApps(invoke: Invoke) {
        val pm: PackageManager = activity.packageManager
        val packages = pm.getInstalledApplications(PackageManager.GET_META_DATA)
        val result = JSArray()

        for (appInfo in packages) {
            val isUserApp = (appInfo.flags and ApplicationInfo.FLAG_SYSTEM) == 0
            val isUpdatedSystemApp = (appInfo.flags and ApplicationInfo.FLAG_UPDATED_SYSTEM_APP) != 0

            if (isUserApp || isUpdatedSystemApp) {
                val app = JSObject()
                app.put("packageName", appInfo.packageName)
                app.put("appName", pm.getApplicationLabel(appInfo).toString())
                app.put("isSystemApp", !isUserApp)
                app.put("isUserApp", isUserApp)

                try {
                    val pkgInfo = pm.getPackageInfo(appInfo.packageName, 0)
                    app.put("versionName", pkgInfo.versionName ?: "")
                    app.put("versionCode", pkgInfo.longVersionCode)
                    app.put("firstInstallTime", pkgInfo.firstInstallTime)
                    app.put("lastUpdateTime", pkgInfo.lastUpdateTime)
                } catch (e: PackageManager.NameNotFoundException) {
                    app.put("versionName", "")
                    app.put("versionCode", 0)
                }

                result.put(app)
            }
        }

        val response = JSObject()
        response.put("apps", result)
        invoke.resolve(response)
    }

    @Command
    fun getAppTvBanner(invoke: Invoke) {
        val packageName = invoke.getArgs().optString("packageName").takeIf { it.isNotEmpty() } ?: run {
            invoke.reject("packageName is required")
            return
        }

        val pm: PackageManager = activity.packageManager
        try {
            val appInfo = pm.getApplicationInfo(packageName, PackageManager.GET_META_DATA)
            // Prefer the TV banner (wide 320x180 image), fall back to the regular launcher icon
            val drawable = appInfo.loadBanner(pm) ?: pm.getApplicationIcon(appInfo)
            val bitmap = drawableToBitmap(drawable)
            val baos = ByteArrayOutputStream()
            bitmap.compress(Bitmap.CompressFormat.PNG, 100, baos)
            val base64 = Base64.encodeToString(baos.toByteArray(), Base64.NO_WRAP)

            val response = JSObject()
            response.put("data", base64)
            response.put("mimeType", "image/png")
            invoke.resolve(response)
        } catch (e: PackageManager.NameNotFoundException) {
            invoke.reject("Package not found: $packageName")
        }
    }

    @Command
    fun getWifiSignal(invoke: Invoke) {
        val wifiManager = activity.applicationContext.getSystemService(Context.WIFI_SERVICE) as WifiManager
        @Suppress("DEPRECATION")
        val wifiInfo = wifiManager.connectionInfo
        val response = JSObject()
        if (wifiInfo != null && wifiInfo.networkId != -1) {
            val rssi = wifiInfo.rssi
            @Suppress("DEPRECATION")
            val level = WifiManager.calculateSignalLevel(rssi, 5)
            response.put("connected", true)
            response.put("rssi", rssi)
            response.put("level", level)
            response.put("ssid", wifiInfo.ssid ?: "")
        } else {
            response.put("connected", false)
            response.put("rssi", 0)
            response.put("level", 0)
            response.put("ssid", "")
        }
        invoke.resolve(response)
    }

    @Command
    fun getBluetoothStatus(invoke: Invoke) {
        val bluetoothManager = activity.getSystemService(Context.BLUETOOTH_SERVICE) as? BluetoothManager
        val adapter = bluetoothManager?.adapter
        val response = JSObject()
        response.put("available", adapter != null)
        response.put("enabled", adapter?.isEnabled ?: false)
        invoke.resolve(response)
    }

    @Command
    fun openSettings(invoke: Invoke) {
        val intent = Intent(Settings.ACTION_SETTINGS)
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        activity.startActivity(intent)
        invoke.resolve(JSObject())
    }

    private fun drawableToBitmap(drawable: Drawable): Bitmap {
        if (drawable is BitmapDrawable) return drawable.bitmap
        val width = if (drawable.intrinsicWidth > 0) drawable.intrinsicWidth else 1
        val height = if (drawable.intrinsicHeight > 0) drawable.intrinsicHeight else 1
        val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
        val canvas = Canvas(bitmap)
        drawable.setBounds(0, 0, canvas.width, canvas.height)
        drawable.draw(canvas)
        return bitmap
    }
}

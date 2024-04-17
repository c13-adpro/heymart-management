package id.ac.ui.cs.advprog.heymartmanagement.controller;

import ch.qos.logback.core.model.Model;
import id.ac.ui.cs.advprog.heymartmanagement.model.Product;
import id.ac.ui.cs.advprog.heymartmanagement.service.ProductService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;

@Controller
@RequestMapping("/product")
public class ProductController {

    @Autowired
    private ProductService service;

    @GetMapping("/create")
    public String createProductPage(Model model){

        return "createProduct";

    }



}
